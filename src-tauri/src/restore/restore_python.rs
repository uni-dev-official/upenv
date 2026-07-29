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

/// Try to restore packages that are not available from PyPI
/// through another installation method.
fn try_special_package_restore(package: &str) -> Result<bool> {
    let package_name = package
        .split("==")
        .next()
        .unwrap_or(package)
        .trim()
        .to_lowercase();

    match package_name.as_str() {
        "ndiff" => {
            println!(
                "Package {} could not be installed through pip.",
                package
            );

            if !command_exists("brew") {
                println!(
                    "Homebrew is unavailable. Cannot restore {} automatically.",
                    package
                );

                return Ok(false);
            }

            println!(
                "Trying to restore {} through Nmap/Homebrew...",
                package
            );

            match run_command("brew", &["install", "nmap"]) {
                Ok(_) => {
                    println!(
                        "Successfully restored {} through Nmap/Homebrew.",
                        package
                    );

                    Ok(true)
                }

                Err(err) => {
                    eprintln!(
                        "Could not restore {} through Nmap/Homebrew: {}",
                        package, err
                    );

                    Ok(false)
                }
            }
        }

        _ => Ok(false),
    }
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

    let restorely_dir = home.join(".restorely");
    let venv_path: PathBuf = restorely_dir.join("python-venv");

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
        println!(
            "Restorely Python virtual environment already exists."
        );
    }

    // ---------------------------------------------------------
    // 3. Upgrade pip inside the virtual environment
    // ---------------------------------------------------------

    println!(
        "Updating pip inside Restorely virtual environment..."
    );

    if let Err(err) = run_command(
        &venv_python_string,
        &["-m", "pip", "install", "--upgrade", "pip"],
    ) {
        eprintln!(
            "Warning: could not upgrade pip: {}",
            err
        );

        eprintln!(
            "Continuing with the existing pip installation..."
        );
    }

    // ---------------------------------------------------------
    // 4. Restore recorded Python packages
    // ---------------------------------------------------------

    let mut installed_count = 0usize;
    let mut skipped_count = 0usize;
    let mut failed_packages: Vec<String> = Vec::new();

    if snapshot.python.packages.is_empty() {
        println!("No Python packages found in snapshot.");
    } else {
        println!(
            "Restoring {} Python packages...",
            snapshot.python.packages.len()
        );

        for package in &snapshot.python.packages {
            println!(
                "--------------------------------------------------"
            );

            println!(
                "Installing Python package: {}",
                package
            );

            // First try normal pip installation.
            match run_command(
                &venv_python_string,
                &["-m", "pip", "install", package],
            ) {
                Ok(_) => {
                    println!(
                        "Successfully installed: {}",
                        package
                    );

                    installed_count += 1;
                }

                Err(pip_error) => {
                    eprintln!(
                        "pip could not install {}.",
                        package
                    );

                    eprintln!(
                        "pip error: {}",
                        pip_error
                    );

                    // Try an alternative installation mechanism.
                    match try_special_package_restore(package) {
                        Ok(true) => {
                            installed_count += 1;
                        }

                        Ok(false) => {
                            skipped_count += 1;
                            failed_packages.push(package.clone());

                            eprintln!(
                                "WARNING: Could not restore {}.",
                                package
                            );

                            eprintln!(
                                "Continuing with the remaining Python packages..."
                            );
                        }

                        Err(err) => {
                            skipped_count += 1;
                            failed_packages.push(package.clone());

                            eprintln!(
                                "WARNING: Alternative restore failed for {}: {}",
                                package, err
                            );

                            eprintln!(
                                "Continuing with the remaining Python packages..."
                            );
                        }
                    }
                }
            }
        }
    }

    // ---------------------------------------------------------
    // 5. Print recorded version information
    // ---------------------------------------------------------

    println!("--------------------------------------------------");

    if let Some(version) = &snapshot.python.python_version {
        println!(
            "Snapshot Python version: {}",
            version
        );
    }

    if let Some(version) = &snapshot.python.pip_version {
        println!(
            "Snapshot pip version: {}",
            version
        );
    }

    // ---------------------------------------------------------
    // 6. Restore summary
    // ---------------------------------------------------------

    println!("--------------------------------------------------");

    println!(
        "Python packages successfully restored: {}",
        installed_count
    );

    println!(
        "Python packages skipped: {}",
        skipped_count
    );

    if !failed_packages.is_empty() {
        println!("Packages that could not be restored:");

        for package in &failed_packages {
            println!("  - {}", package);
        }
    }

    println!(
        "Python environment restored to: {}",
        venv_path_string
    );

    println!("Python restore completed.");
    println!("==========================");

    Ok(())
}