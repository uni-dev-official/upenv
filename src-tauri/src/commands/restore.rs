//! Restore orchestration command — drives the restore pipeline and emits
//! progress events back to the frontend. Implemented in Milestone 8.

#[tauri::command]
pub async fn run_restore(app_handle: tauri::AppHandle, snapshot_id: String) -> Result<(), String> {
    crate::restore::restore::run(app_handle, &snapshot_id)
        .await
        .map_err(|e| e.to_string())
}
