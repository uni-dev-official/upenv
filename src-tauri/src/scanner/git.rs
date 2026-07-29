//! Collects Git version and ~/.gitconfig contents.
//! Credentials are not collected.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitInfo {
    pub version: Option<String>,
    pub gitconfig: Option<String>,
}

pub async fn scan_git() -> Result<GitInfo> {
    let version = match Command::new("git")
        .arg("--version")
        .output()
    {
        Ok(output) if output.status.success() => {
            Some(
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string(),
            )
        }

        _ => None,
    };

    let home = std::env::var("HOME")
        .map_err(|_| anyhow!("HOME environment variable not found"))?;

    let gitconfig_path = PathBuf::from(home).join(".gitconfig");

    let gitconfig = match fs::read_to_string(&gitconfig_path) {
        Ok(contents) if !contents.trim().is_empty() => Some(contents),
        _ => None,
    };

    Ok(GitInfo {
        version,
        gitconfig,
    })
}

