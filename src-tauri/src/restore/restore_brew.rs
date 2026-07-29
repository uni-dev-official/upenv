//! Installs Homebrew itself (if missing) and restores
//! recorded Homebrew packages and casks.

use anyhow::{anyhow, Result};
use std::process::Command;

use crate::models::snapshot::Snapshot;

fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {}", command)])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn run_command(command: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(command)
        .args(args)
        .status()
        .map_err(|e| anyhow!("Failed to run {}: {}", command, e))?;

    if !status.success() {
        return Err(anyhow!(
            "{} {:?} failed with status {}",
            command,
            args,
            status
        ));
    }

    Ok(())
}

pub async fn install_homebrew_and_packages(
    snapshot: &Snapshot,
) -> Result<()> {
    println!("===== HOMEBREW RESTORE =====");

    // --------------------------------------------------
    // 1. Install Homebrew if missing
    // --------------------------------------------------

    if !command_exists("brew") {
        println!("Homebrew not found. Installing Homebrew...");

        let status = Command::new("/bin/bash")
            .arg("-c")
            .arg(
                r#"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"#,
            )
            .status()
            .map_err(|e| anyhow!("Failed to start Homebrew installer: {}", e))?;

        if !status.success() {
            return Err(anyhow!("Homebrew installation failed"));
        }

        println!("Homebrew installed.");
    } else {
        println!("Homebrew is already installed.");
    }

    // --------------------------------------------------
    // 2. Update Homebrew
    // --------------------------------------------------

    println!("Updating Homebrew...");

    let _ = Command::new("brew")
        .args(["update"])
        .status();

    // --------------------------------------------------
    // 3. Restore formula packages
    // --------------------------------------------------

    let mut installed_packages = 0;
    let mut skipped_packages = 0;

    for package in &snapshot.brew_packages {
        if package.trim().is_empty() {
            continue;
        }

        let installed = Command::new("brew")
            .args(["list", "--formula", package])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if installed {
            println!("Already installed: {}", package);
            skipped_packages += 1;
            continue;
        }

        println!("Installing Homebrew package: {}", package);

        match run_command("brew", &["install", package]) {
            Ok(_) => {
                println!("Installed: {}", package);
                installed_packages += 1;
            }

            Err(error) => {
                eprintln!(
                    "Failed to install Homebrew package {}: {}",
                    package, error
                );
            }
        }
    }

    // --------------------------------------------------
    // 4. Restore casks
    // --------------------------------------------------

    let mut installed_casks = 0;
    let mut skipped_casks = 0;

    for cask in &snapshot.brew_casks {
        if cask.trim().is_empty() {
            continue;
        }

        let installed = Command::new("brew")
            .args(["list", "--cask", cask])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if installed {
            println!("Already installed cask: {}", cask);
            skipped_casks += 1;
            continue;
        }

        println!("Installing Homebrew cask: {}", cask);

        match run_command("brew", &["install", "--cask", cask]) {
            Ok(_) => {
                println!("Installed cask: {}", cask);
                installed_casks += 1;
            }

            Err(error) => {
                eprintln!(
                    "Failed to install cask {}: {}",
                    cask, error
                );
            }
        }
    }

    println!(
        "Homebrew restore complete. Packages: {} installed, {} skipped. Casks: {} installed, {} skipped.",
        installed_packages,
        skipped_packages,
        installed_casks,
        skipped_casks
    );

    println!("============================");

    Ok(())
}
