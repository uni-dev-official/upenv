//! Restores Git configuration (~/.gitconfig).

use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;

use crate::models::snapshot::Snapshot;

pub async fn restore_git_config(snapshot: &Snapshot) -> Result<()> {
    println!("===== GIT RESTORE =====");

    let Some(gitconfig) = &snapshot.git.gitconfig else {
        println!("No Git configuration found in snapshot.");
        println!("======================");
        return Ok(());
    };

    if gitconfig.trim().is_empty() {
        println!("Git configuration is empty.");
        println!("======================");
        return Ok(());
    }

    let home = std::env::var("HOME").map_err(|_| anyhow!("HOME environment variable not found"))?;

    let gitconfig_path = PathBuf::from(home).join(".gitconfig");

    println!(
        "Restoring Git configuration to: {}",
        gitconfig_path.display()
    );

    fs::write(&gitconfig_path, gitconfig)
        .map_err(|e| anyhow!("Failed to write ~/.gitconfig: {}", e))?;

    println!("Git configuration restored.");
    println!("======================");

    Ok(())
}
