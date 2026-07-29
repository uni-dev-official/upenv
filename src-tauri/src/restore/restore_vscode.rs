//! Installs VS Code and restores VS Code extensions.
//! Implemented in Milestone 8.

use anyhow::{anyhow, Result};
use std::process::Command;

use crate::models::snapshot::Snapshot;

pub async fn restore_vscode(snapshot: &Snapshot) -> Result<()> {
    println!("===== VS CODE RESTORE =====");

    // --------------------------------------------------
    // 1. Install VS Code if it isn't already installed
    // --------------------------------------------------

    println!("Checking for VS Code...");

    let vscode_check = Command::new("code")
        .arg("--version")
        .output();

    if vscode_check.is_err() {
        println!("VS Code CLI not found. Installing VS Code with Homebrew...");

        let brew_install = Command::new("brew")
            .args(["install", "--cask", "visual-studio-code"])
            .status()
            .map_err(|e| anyhow!("Failed to run Homebrew: {}", e))?;

        if !brew_install.success() {
            return Err(anyhow!(
                "Homebrew failed to install Visual Studio Code"
            ));
        }
    } else {
        println!("VS Code is already installed.");
    }

    // --------------------------------------------------
    // 2. Check whether there are extensions to restore
    // --------------------------------------------------

    if snapshot.vscode_extensions.is_empty() {
        println!("No VS Code extensions found in snapshot.");
        println!("============================");

        return Ok(());
    }

    println!(
        "Found {} VS Code extensions to restore.",
        snapshot.vscode_extensions.len()
    );

    // --------------------------------------------------
    // 3. Install each extension
    // --------------------------------------------------

    let mut installed = 0;
    let mut skipped = 0;

    for extension in &snapshot.vscode_extensions {
        println!("Installing VS Code extension: {}", extension);

        let result = Command::new("code")
            .args(["--install-extension", extension])
            .status();

        match result {
            Ok(status) if status.success() => {
                println!("Successfully installed: {}", extension);
                installed += 1;
            }

            Ok(status) => {
                eprintln!(
                    "Failed to install {}. Exit status: {}",
                    extension, status
                );
            }

            Err(error) => {
                eprintln!(
                    "Could not run VS Code CLI for {}: {}",
                    extension, error
                );
            }
        }
    }

    // --------------------------------------------------
    // 4. Summary
    // --------------------------------------------------

    for extension in &snapshot.vscode_extensions {
        if extension.trim().is_empty() {
            skipped += 1;
        }
    }

    println!(
        "VS Code restore complete. Installed: {}. Skipped: {}.",
        installed, skipped
    );

    println!("============================");

    Ok(())
}
