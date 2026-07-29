//! Restores Python and pip packages.
//! Implemented in Milestone 8.

use anyhow::{anyhow, Result};
use std::process::Command;

use crate::models::snapshot::Snapshot;

fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {}", command))
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn run_command(command: &str, args: &[&str]) -> Result<()> {
    println!("Running: {} {}", command, args.join(" "));

    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| anyhow!("Failed to execute {}: {}", command, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        return Err(anyhow!(
            "Command failed: {} {}\n{}",
            command,
            args.join(" "),
            stderr
        ));
    }

    Ok(())
}

pub async fn restore_python(snapshot: &Snapshot) -> Result<()> {
    println!("===== PYTHON RESTORE =====");

    // ---------------------------------------------------------
    // 1. Make sure Python is installed
    // ---------------------------------------------------------

    if !command_exists("python3") {
        println!("Python 3 is not installed.");

        if !command_exists("brew") {
            return Err(anyhow!(
                "Python is not installed and Homebrew is unavailable."
            ));
        }

        println!("Installing Python through Homebrew...");

        run_command("brew", &["install", "python"])?;
    } else {
        println!("Python 3 is already installed.");
    }

    // ---------------------------------------------------------
    // 2. Make sure pip is available
    // ---------------------------------------------------------

    if !command_exists("pip3") {
        println!("pip3 is not available.");

        run_command(
            "python3",
            &["-m", "ensurepip", "--upgrade"],
        )
        .ok();
    }

    // ---------------------------------------------------------
    // 3. Restore Python packages
    // ---------------------------------------------------------

    if snapshot.python.packages.is_empty() {
        println!("No Python packages found in snapshot.");
    } else {
        println!(
            "Restoring {} Python packages...",
            snapshot.python.packages.len()
        );

        for package in &snapshot.python.packages {
            println!("Installing Python package: {}", package);

            run_command(
                "python3",
                &["-m", "pip", "install", package],
            )?;
        }
    }

    // ---------------------------------------------------------
    // 4. Show recorded versions
    // ---------------------------------------------------------

    if let Some(version) = &snapshot.python.python_version {
        println!("Snapshot Python version: {}", version);
    }

    if let Some(version) = &snapshot.python.pip_version {
        println!("Snapshot pip version: {}", version);
    }

    println!("Python restore completed.");
    println!("==========================");

    Ok(())
}
