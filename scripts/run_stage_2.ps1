# Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the MIT License
# SPDX-License-Identifier: MIT

$ErrorActionPreference = "Stop"
Push-Location (Split-Path $PSScriptRoot -Parent)

function Invoke-Step {
    param([string]$Label, [scriptblock]$Command)
    Write-Host "$Label..." -ForegroundColor Cyan
    & $Command
    if ($LASTEXITCODE -ne 0) {
        Write-Host "`nFailed: $Label (exit code $LASTEXITCODE)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

# ---------------------------------------------------------------------------
# Self-analysis: braintax on braintax
# ---------------------------------------------------------------------------

Invoke-Step "cargo-braintax self-analysis" {
    cargo run --package cargo-braintax4rust -- --json | Out-Null
}

# ---------------------------------------------------------------------------
# CRAP gate
# ---------------------------------------------------------------------------

Invoke-Crap4RustGate "CRAP core" @("core") -ExcludePaths @("tests/fixtures")

Write-Host "`nbraintax Stage 2 passed!" -ForegroundColor Green
Pop-Location
exit 0
