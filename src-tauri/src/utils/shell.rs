//! Wraps tokio::process::Command for running shell commands used by both
//! scanners (read-only, e.g. `git --version`) and restore steps (mutating,
//! e.g. `brew install`). Centralized here so error handling/logging is
//! consistent everywhere a subprocess is spawned.

use anyhow::{Context, Result};
use tokio::process::Command;

/// Runs a command and returns trimmed stdout on success.
pub async fn run(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .await
        .with_context(|| format!("failed to spawn `{program}`"))?;

    if !output.status.success() {
        anyhow::bail!(
            "`{program} {}` exited with status {}: {}",
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Returns true if a binary exists on PATH (via `which`).
pub async fn command_exists(program: &str) -> bool {
    run("which", &[program]).await.is_ok()
}
