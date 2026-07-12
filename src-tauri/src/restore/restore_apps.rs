//! Installs applications where possible (via Homebrew casks when available).
//! Implemented in Milestone 8.

use anyhow::Result;

pub async fn install_applications() -> Result<()> {
    // Milestone 8: map app name -> brew cask, shell out to `brew install --cask`.
    Ok(())
}
