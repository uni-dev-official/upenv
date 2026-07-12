//! Resolves well-known paths (home dir, ~/.ssh, ~/.gitconfig, etc.) using `dirs`.

use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("could not resolve home directory")
}

pub fn gitconfig_path() -> Result<PathBuf> {
    Ok(home_dir()?.join(".gitconfig"))
}

pub fn zshrc_path() -> Result<PathBuf> {
    Ok(home_dir()?.join(".zshrc"))
}

pub fn bashrc_path() -> Result<PathBuf> {
    Ok(home_dir()?.join(".bashrc"))
}

pub fn ssh_dir_exists() -> Result<bool> {
    Ok(home_dir()?.join(".ssh").exists())
}

pub fn applications_dir() -> PathBuf {
    PathBuf::from("/Applications")
}
