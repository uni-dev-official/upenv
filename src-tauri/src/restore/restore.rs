//! Orchestrates the full restore pipeline in order, emitting progress
//! events to the frontend after each step. Implemented in Milestone 8.

use anyhow::Result;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RestoreProgress {
    pub step: String,
    pub message: String,
    pub done: bool,
}

fn emit_progress(app: &AppHandle, step: &str, message: &str, done: bool) {
    let _ = app.emit(
        "restore://progress",
        RestoreProgress {
            step: step.to_string(),
            message: message.to_string(),
            done,
        },
    );
}

/// Runs the restore pipeline for a given device ID, step by step:
/// Homebrew -> packages -> apps -> Git -> shell -> VS Code -> Node -> Python.
pub async fn run(app: AppHandle, device_id: &str, access_token: &str) -> Result<()> {
    emit_progress(&app, "fetch", "Downloading snapshot...", false);
    let snapshot = crate::services::supabase_storage::fetch_latest_snapshot_for_device(
        device_id,
        access_token,
    )
    .await?;

    emit_progress(&app, "brew", "Installing Homebrew...", false);
    super::restore_brew::install_homebrew_and_packages().await?;

    emit_progress(&app, "git", "Restoring Git configuration...", false);
    super::restore_git::restore_git_config().await?;

    emit_progress(&app, "apps", "Installing applications...", false);
    let apps_summary =
        super::restore_apps::install_applications(&snapshot.applications).await?;
    emit_progress(
        &app,
        "apps",
        &format!(
            "Applications restore complete. Installed: {}. Skipped (already installed): {}.",
            apps_summary.installed, apps_summary.skipped
        ),
        false,
    );

    emit_progress(&app, "vscode", "Installing VS Code and extensions...", false);
    super::restore_vscode::restore_vscode(&snapshot).await?;

    emit_progress(&app, "node", "Restoring Node...", false);
    super::restore_node::restore_node().await?;

    emit_progress(&app, "python", "Restoring Python...", false);
    super::restore_python::restore_python().await?;

    emit_progress(&app, "done", "Finished.", true);
    Ok(())
}
