//! Collects installed Homebrew packages and casks.

use anyhow::{anyhow, Result};
use std::process::Command;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BrewInventory {
    pub packages: Vec<String>,
    pub casks: Vec<String>,
}

fn run_command(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| anyhow!("Failed to run {}: {}", command, e))?;

    if !output.status.success() {
        return Err(anyhow!(
            "{} failed: {}",
            command,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn scan_brew() -> Result<BrewInventory> {
    // Homebrew isn't installed.
    let brew_check = Command::new("brew").arg("--version").output();

    if brew_check.is_err() {
        println!("Homebrew is not installed.");
        return Ok(BrewInventory::default());
    }

    let formula_output = run_command("brew", &["list", "--formula"])?;
    let cask_output = run_command("brew", &["list", "--cask"])?;

    let packages = formula_output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();

    let casks = cask_output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();

    Ok(BrewInventory { packages, casks })
}
