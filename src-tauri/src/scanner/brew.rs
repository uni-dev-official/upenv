//! Collects installed Homebrew packages and casks via `brew list`.
//! Implemented in Milestone 4.

use anyhow::Result;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BrewInventory {
    pub packages: Vec<String>,
    pub casks: Vec<String>,
}

pub async fn scan_brew() -> Result<BrewInventory> {
    // Milestone 4: shell out to `brew list --formula` and `brew list --cask`.
    Ok(BrewInventory::default())
}
