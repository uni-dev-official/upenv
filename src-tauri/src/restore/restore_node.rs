//! Restores Node.js and global npm packages.

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

pub async fn restore_node(snapshot: &Snapshot) -> Result<()> {
    println!("===== NODE RESTORE =====");

    // --------------------------------------------------
    // 1. Install Node if missing
    // --------------------------------------------------

    if !command_exists("node") {
        println!("Node.js not found.");

        if !command_exists("brew") {
            return Err(anyhow!(
                "Homebrew is not installed, so Node.js cannot be installed."
            ));
        }

        println!("Installing Node.js through Homebrew...");

        run_command("brew", &["install", "node"])?;

        println!("Node.js installed.");
    } else {
        println!("Node.js is already installed.");
    }

    // --------------------------------------------------
    // 2. Restore global npm packages
    // --------------------------------------------------

    let mut installed = 0;
    let mut skipped = 0;

    for package in &snapshot.node.global_packages {
        if package.trim().is_empty() {
            continue;
        }

        let already_installed = Command::new("npm")
            .args(["list", "-g", "--depth=0", package])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if already_installed {
            println!("Already installed: {}", package);
            skipped += 1;
            continue;
        }

        println!("Installing global npm package: {}", package);

        match run_command("npm", &["install", "-g", package]) {
            Ok(_) => {
                println!("Installed: {}", package);
                installed += 1;
            }

            Err(error) => {
                eprintln!("Failed to install npm package {}: {}", package, error);
            }
        }
    }

    println!(
        "Node restore complete. Installed: {}. Skipped: {}.",
        installed, skipped
    );

    println!("=======================");

    Ok(())
}
