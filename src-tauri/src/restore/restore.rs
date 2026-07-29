//! Orchestrates the full restore pipeline in order,
//! emitting progress events to the frontend.

use anyhow::Result;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RestoreProgress {
    pub step: String,
    pub message: String,
    pub done: bool,
    pub current: Option<usize>,
    pub total: Option<usize>,
}

fn emit_progress(
    app: &AppHandle,
    step: &str,
    message: &str,
    done: bool,
    current: Option<usize>,
    total: Option<usize>,
) {
    let _ = app.emit(
        "restore://progress",
        RestoreProgress {
            step: step.to_string(),
            message: message.to_string(),
            done,
            current,
            total,
        },
    );
}

/// Runs the complete Restorely restore pipeline.
pub async fn run(
    app: AppHandle,
    device_id: &str,
    access_token: &str,
) -> Result<()> {
    // =========================================================
    // 1. DOWNLOAD SNAPSHOT
    // =========================================================

    emit_progress(
        &app,
        "fetch",
        "Downloading snapshot...",
        false,
        None,
        None,
    );

    let snapshot =
        crate::services::supabase_storage::fetch_latest_snapshot_for_device(
            device_id,
            access_token,
        )
        .await?;

    emit_progress(
        &app,
        "fetch",
        "Snapshot downloaded successfully.",
        true,
        None,
        None,
    );

    // =========================================================
    // 2. HOMEBREW
    // =========================================================

    emit_progress(
        &app,
        "brew",
        "Installing Homebrew and packages...",
        false,
        None,
        None,
    );

    super::restore_brew::install_homebrew_and_packages(&snapshot).await?;

    emit_progress(
        &app,
        "brew",
        "Homebrew packages restored.",
        true,
        None,
        None,
    );

    // =========================================================
    // 3. GIT
    // =========================================================

    emit_progress(
        &app,
        "git",
        "Restoring Git configuration...",
        false,
        None,
        None,
    );

    super::restore_git::restore_git_config(&snapshot).await?;

    emit_progress(
        &app,
        "git",
        "Git configuration restored.",
        true,
        None,
        None,
    );

    // =========================================================
    // 4. APPLICATIONS
    // =========================================================

    let total_apps = snapshot.applications.len();

    if total_apps == 0 {
        emit_progress(
            &app,
            "apps",
            "No applications found in snapshot.",
            true,
            Some(0),
            Some(0),
        );
    } else {
        emit_progress(
            &app,
            "apps",
            "Preparing applications...",
            false,
            Some(0),
            Some(total_apps),
        );

        let app_handle = app.clone();

        let apps_summary =
            super::restore_apps::install_applications(
                &snapshot.applications,
                move |current, total, app_name, status| {
                    let message = match status {
                        "installing" => {
                            format!("Installing {}...", app_name)
                        }

                        "installed" => {
                            format!("✓ {} installed", app_name)
                        }

                        "already installed" => {
                            format!("✓ {} already installed", app_name)
                        }

                        "failed" => {
                            format!("✗ {} failed", app_name)
                        }

                        _ => {
                            format!("{} — {}", app_name, status)
                        }
                    };

                    emit_progress(
                        &app_handle,
                        "apps",
                        &message,
                        false,
                        Some(current),
                        Some(total),
                    );
                },
            )
            .await?;

        emit_progress(
            &app,
            "apps",
            &format!(
                "Applications complete. Installed: {}. Already installed: {}.",
                apps_summary.installed,
                apps_summary.skipped
            ),
            true,
            Some(total_apps),
            Some(total_apps),
        );
    }

    // =========================================================
    // 5. VS CODE
    // =========================================================

    emit_progress(
        &app,
        "vscode",
        "Restoring VS Code and extensions...",
        false,
        None,
        None,
    );

    super::restore_vscode::restore_vscode(&snapshot).await?;

    emit_progress(
        &app,
        "vscode",
        "VS Code restoration complete.",
        true,
        None,
        None,
    );

    // =========================================================
    // 6. NODE
    // =========================================================

    emit_progress(
        &app,
        "node",
        "Restoring Node.js and npm packages...",
        false,
        None,
        None,
    );

    super::restore_node::restore_node(&snapshot).await?;

    emit_progress(
        &app,
        "node",
        "Node.js restoration complete.",
        true,
        None,
        None,
    );

    // =========================================================
    // 7. PYTHON
    // =========================================================

    emit_progress(
        &app,
        "python",
        "Restoring Python and packages...",
        false,
        None,
        None,
    );

    super::restore_python::restore_python(&snapshot).await?;

    emit_progress(
        &app,
        "python",
        "Python restoration complete.",
        true,
        None,
        None,
    );

    // =========================================================
    // 8. FINISHED
    // =========================================================

    emit_progress(
        &app,
        "done",
        "Restore completed successfully.",
        true,
        None,
        None,
    );

    Ok(())
}