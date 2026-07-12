//! Installs VS Code (via brew cask) and re-installs extensions.
//! Implemented in Milestone 8.

use anyhow::Result;

pub async fn restore_vscode() -> Result<()> {
    // Milestone 8: `brew install --cask visual-studio-code`, then
    // `code --install-extension <id>` per extension.
    Ok(())
}
