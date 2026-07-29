//! Collects Python version, pip version, and installed packages.
//! Implemented in Milestone 4.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PythonInfo {
    pub python_version: Option<String>,
    pub pip_version: Option<String>,
    pub packages: Vec<String>,
}

/// Runs a command and returns stdout if successful.
fn run_command(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let output = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}

pub async fn scan_python() -> Result<PythonInfo> {
    // ---------------------------------------------------------
    // Python version
    // ---------------------------------------------------------

    let python_version = run_command("python3", &["--version"]);

    // ---------------------------------------------------------
    // pip version
    // ---------------------------------------------------------

    let pip_version = run_command("python3", &["-m", "pip", "--version"]);

    // ---------------------------------------------------------
    // Installed pip packages
    // ---------------------------------------------------------

    let mut packages = Vec::new();

    if let Some(output) = run_command(
        "python3",
        &["-m", "pip", "list", "--format=freeze"],
    ) {
        for line in output.lines() {
            let package = line.trim();

            if !package.is_empty() {
                packages.push(package.to_string());
            }
        }
    }

    Ok(PythonInfo {
        python_version,
        pip_version,
        packages,
    })
}
