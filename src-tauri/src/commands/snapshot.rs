use crate::models::snapshot::Snapshot;

#[tauri::command]
pub async fn scan_snapshot() -> Result<Snapshot, String> {
    crate::scanner::snapshot::build_snapshot()
        .await
        .map_err(|e| e.to_string())
}


#[tauri::command]
pub async fn upload_snapshot(
    user_id: String,
    device_id: String,
    device_name: String,
    access_token: String,
) -> Result<String, String> {

    let snapshot = crate::scanner::snapshot::build_snapshot()
        .await
        .map_err(|e| e.to_string())?;

    match crate::services::supabase_storage::upload_snapshot(
        &user_id,
        &device_id,
        &device_name,
        &access_token,
        &snapshot,
    )
    .await
    {
        Ok(id) => Ok(id),
        Err(e) => {
            eprintln!("========== UPLOAD ERROR ==========");
            eprintln!("{:?}", e);
            eprintln!("==================================");

            Err(e.to_string())
        }
    }
}