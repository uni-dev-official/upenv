//! Orchestrates a full machine scan and snapshot generation.
//! Implemented in Milestone 4 (Scanner Engine) and Milestone 5 (Snapshot).

use crate::models::snapshot::Snapshot;

#[tauri::command]
pub async fn run_full_scan() -> Result<Snapshot, String> {
    crate::scanner::snapshot::build_snapshot()
        .await
        .map_err(|e| e.to_string())
}
