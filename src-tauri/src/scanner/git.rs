//! Collects Git version and ~/.gitconfig contents (no credentials).
//! Implemented in Milestone 4.

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitInfo {
    pub version: Option<String>,
    pub gitconfig: Option<String>,
}

pub async fn scan_git() -> Result<GitInfo> {
    // Milestone 4: `git --version` + read ~/.gitconfig via utils::fs_paths.
    Ok(GitInfo::default())
}