//! Devices list command — fetches saved snapshots/devices from Supabase.
//! Implemented in Milestone 7 (Devices Page).

use crate::models::device::Device;

#[tauri::command]
pub async fn list_devices(
    user_id: String,
    access_token: String,
) -> Result<Vec<Device>, String> {
    crate::services::supabase_storage::fetch_devices(&user_id, &access_token)
        .await
        .map_err(|e| e.to_string())
}
    