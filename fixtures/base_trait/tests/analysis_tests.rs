// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use anyhow::Result;
use braintax::app::App;
use braintax::braintax_report::BraintaxReport;
use braintax::config::Config;
use braintax::default_scorer::DefaultScorer;
use braintax::traits::reporter::Reporter;
use braintax::traits::walk::Walk;

#[derive(Debug)]
struct TestWalk {
    files: Vec<(PathBuf, String)>,
}

impl Walk for TestWalk {
    fn rust_files(&self) -> Result<Vec<(PathBuf, String)>> {
        Ok(self.files.clone())
    }
}

struct CaptureReporter {
    captured: std::sync::Mutex<String>,
}

impl Reporter for CaptureReporter {
    fn render(&self, report: &BraintaxReport) -> Result<String> {
        Ok(serde_json::to_string_pretty(report)?)
    }
    fn write(&self, report: &BraintaxReport) -> Result<()> {
        let json = serde_json::to_string_pretty(report)?;
        *self.captured.lock().unwrap() = json;
        Ok(())
    }
}

#[test]
fn trait_impl_body_has_cc_18() {
    // Arrange
    let src = r#"
pub trait Computer {
    fn compute(&self, x: i32) -> i32;
}
pub struct MyComputer;
impl Computer for MyComputer {
    fn compute(&self, x: i32) -> i32 {
        let mut result = 0;
        for i in 0..10 {
            if i % 2 == 0 {
                if i % 3 == 0 {
                    result += i * 2;
                } else if i % 3 == 1 {
                    result += i;
                } else {
                    result -= i;
                }
            }
            match i {
                0 | 1 => result += 1,
                2 | 3 => result += 2,
                4 | 5 => result += 3,
                _ => {
                    if result > 0 {
                        return result;
                    }
                }
            }
            if i > 5 && i < 8 {
                loop {
                    result += 1;
                    if result > 20 {
                        break;
                    }
                    if result == 15 {
                        continue;
                    }
                }
            }
        }
        result
    }
}
"#;

    // Act
    let walk = TestWalk {
        files: vec![(PathBuf::from("src/lib.rs"), src.to_string())],
    };
    let reporter = CaptureReporter {
        captured: std::sync::Mutex::new(String::new()),
    };
    let app = App::with_deps(
        walk,
        DefaultScorer::new(),
        reporter,
        Config {
            path: PathBuf::from("."),
            json: true,
            threshold: None,
            top: 10,
        },
    );
    app.run().unwrap();
    let json = app.reporter().captured.lock().unwrap().clone();
    let report: BraintaxReport = serde_json::from_str(&json).unwrap();

    // Assert
    assert_eq!(report.overall.total_functions, 1);
    assert_eq!(report.overall.max_cyclomatic, 18);
    assert_eq!(report.functions[0].name, "compute");
    assert_eq!(report.functions[0].cyclomatic, 18);
}
