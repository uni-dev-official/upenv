//! System info command — thin wrapper around `scanner::system`.
//! This is the first real end-to-end command (frontend -> Rust -> back)
//! used to validate the Milestone 1 build.

use crate::models::system_info::SystemInfo;

#[tauri::command]
pub async fn scan_system() -> Result<SystemInfo, String> {
    crate::scanner::system::collect_system_info()
        .await
        .map_err(|e| e.to_string())
}
