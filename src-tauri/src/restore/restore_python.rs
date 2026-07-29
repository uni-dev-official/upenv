//! Restores Python and pip packages.
//! Implemented in Milestone 8.

use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::models::snapshot::Snapshot;

fn run_command(command: &str, args: &[&str]) -> Result<()> {
    println!("Running: {} {}", command, args.join(" "));

    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| anyhow!("Failed to execute {}: {}", command, e))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        return Err(anyhow!(
            "Command failed: {} {}\nstdout: {}\nstderr: {}",
            command,
            args.join(" "),
            stdout,
            stderr
        ));
    }

    Ok(())
}

fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {}", command))
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub async fn restore_python(snapshot: &Snapshot) -> Result<()> {
    println!("===== PYTHON RESTORE =====");

    // ---------------------------------------------------------
    // 1. Make sure Python 3 exists
    // ---------------------------------------------------------

    if !command_exists("python3") {
        println!("Python 3 is not installed.");

        if !command_exists("brew") {
            return Err(anyhow!(
                "Python 3 is not installed and Homebrew is unavailable."
            ));
        }

        println!("Installing Python through Homebrew...");

        run_command("brew", &["install", "python"])?;
    } else {
        println!("Python 3 is already installed.");
    }

    // ---------------------------------------------------------
    // 2. Create Restorely's dedicated virtual environment
    // ---------------------------------------------------------

    let home = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;

    let venv_path: PathBuf = home.join(".restorely").join("python-venv");

    let venv_path_string = venv_path
        .to_string_lossy()
        .to_string();

    let venv_python = venv_path.join("bin").join("python3");

    let venv_python_string = venv_python
        .to_string_lossy()
        .to_string();

    if !venv_python.exists() {
        println!(
            "Creating Restorely Python virtual environment at {}",
            venv_path_string
        );

        run_command(
            "python3",
            &["-m", "venv", &venv_path_string],
        )?;
    } else {
        println!("Restorely Python virtual environment already exists.");
    }

    // ---------------------------------------------------------
    // 3. Upgrade pip inside the virtual environment
    // ---------------------------------------------------------

    println!("Updating pip inside Restorely virtual environment...");

    run_command(
        &venv_python_string,
        &["-m", "pip", "install", "--upgrade", "pip"],
    )?;

    // ---------------------------------------------------------
    // 4. Restore recorded packages
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
                &venv_python_string,
                &["-m", "pip", "install", package],
            )?;
        }
    }

    // ---------------------------------------------------------
    // 5. Print recorded version information
    // ---------------------------------------------------------

    if let Some(version) = &snapshot.python.python_version {
        println!("Snapshot Python version: {}", version);
    }

    if let Some(version) = &snapshot.python.pip_version {
        println!("Snapshot pip version: {}", version);
    }

    println!(
        "Python environment restored to: {}",
        venv_path_string
    );

    println!("Python restore completed.");
    println!("==========================");

    Ok(())
}
