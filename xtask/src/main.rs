// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::braintax_self_gate::BraintaxSelfGate;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::dry_gate::DryGate;
use xtask::gates::gate::Gate;
use xtask::gates::iceberg_gate::IcebergGate;
use xtask::gates::shear_gate::ShearGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::process::system_command_runner::SystemCommandRunner;

const CLI_PACKAGE: &str = "cargo-braintax4rust";
const CORE_PACKAGE: &str = "braintax4rust-core";
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "20";
// core/ and cli/, each on its own: the ceiling is set against what the tool's
// own code scores rather than against the fixture-inflated 8.0 the old default
// produced.
const BRAINTAX_CORE_TARGET: &str = "core";
const BRAINTAX_CLI_TARGET: &str = "cli";
const BRAINTAX_CEILING: &str = "5.0";
// Every tree dry4rust must not judge, below the workspace root it scans:
// tests repeat their arrangement by design, and xtask, the fixtures and
// validation are not published.
const DRY_EXCLUDES: [&[&str]; 5] = [
    &["xtask"],
    &["fixture"],
    &["validation"],
    &["core", "tests"],
    &["cli", "tests"],
];
const DRY_BASELINE: &str = "dry4rust-baseline.json";
const DRY_MIN_NODES: &str = "25";

// Reading the real process argv and wiring the concrete runner are the two
// things no test can reach, so they are all this binary does.
fn main() -> ExitCode {
    match args().nth(1).as_deref() {
        Some("stage2") => run_stage2(),
        _ => {
            eprintln!("usage: cargo xtask stage2");
            ExitCode::FAILURE
        }
    }
}

fn run_stage2() -> ExitCode {
    let manifest_path = workspace_manifest_path();
    let runner = SystemCommandRunner::new();
    let parser = CrapReportParser::new();

    // Every member, so the gate checks what a hand-run of `cargo stern4rust`
    // checks: xtask, because it is the crate that runs the gates, and the
    // fixtures, whose source files that break a rule by construction
    // stern4rust.toml excludes by name. Left out, the fixtures broke
    // named-subject six times with the gate green.
    let stern = SternGate::new(&runner, manifest_path.clone(), Vec::new());

    // Second: every declared dependency used, in the section that uses it. Its
    // corrections edit manifests only, so they move nothing the gates behind it
    // measure, and it costs a second.
    let shear = ShearGate::new(&runner, manifest_path.clone());

    // Third, for the same reason stern4rust is first: removing a duplicate
    // moves code between files and functions, which changes what every gate
    // behind it measures -- the braintax self-analysis included. One run over
    // the workspace with the trees it must not judge left out, so a copy from
    // one published crate into another is seen, against a baseline of what was
    // already duplicated when the gate arrived, so it fails on what a change
    // adds. The floor leaves out the one-call `Visit` overrides and the one-
    // and two-field constructors, whose sameness is the trait's or the
    // struct's rather than a copy.
    let root = workspace_root();
    let dry = DryGate::new(
        &runner,
        root.to_string_lossy().into_owned(),
        DRY_EXCLUDES
            .iter()
            .map(|exclude| below(&root, exclude).to_string_lossy().into_owned())
            .collect(),
        root.join(DRY_BASELINE).to_string_lossy().into_owned(),
        String::from(DRY_MIN_NODES),
    );

    // The binary is cli's; it measures each published crate's source in turn.
    let braintax_core = BraintaxSelfGate::new(
        &runner,
        String::from(CLI_PACKAGE),
        String::from(BRAINTAX_CORE_TARGET),
        String::from(BRAINTAX_CEILING),
    );
    let braintax_cli = BraintaxSelfGate::new(
        &runner,
        String::from(CLI_PACKAGE),
        String::from(BRAINTAX_CLI_TARGET),
        String::from(BRAINTAX_CEILING),
    );

    // The two published crates. CRAP scores source functions against their
    // coverage, and the fixtures carry one acceptance test each rather than a
    // mirrored suite.
    let published = vec![String::from(CORE_PACKAGE), String::from(CLI_PACKAGE)];

    let crap = CrapGate::new(
        &runner,
        &parser,
        manifest_path.clone(),
        published.clone(),
        String::from(CRAP_THRESHOLD),
    );
    let twin = TwinGate::new(&runner, manifest_path.clone(), published.clone());
    let iceberg = IcebergGate::new(
        &runner,
        manifest_path,
        published,
        String::from(ICEBERG_THRESHOLD),
    );

    let gates: Vec<&dyn Gate> = vec![
        &stern,
        &shear,
        &dry,
        &braintax_core,
        &braintax_cli,
        &crap,
        &twin,
        &iceberg,
    ];

    match Stage2::new(gates).run() {
        Ok(()) => {
            println!("\nbraintax Stage 2 passed!");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn workspace_manifest_path() -> String {
    workspace_root()
        .join("Cargo.toml")
        .to_string_lossy()
        .into_owned()
}

// Joined a component at a time, so the path carries the platform's separator
// throughout -- dry4rust matches its exclusions as text.
fn below(root: &Path, components: &[&str]) -> PathBuf {
    components
        .iter()
        .fold(root.to_path_buf(), |path, component| path.join(component))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
        .to_path_buf()
}
