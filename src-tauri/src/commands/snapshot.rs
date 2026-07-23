use crate::models::snapshot::Snapshot;
use crate::services::device_service::ensure_device;

#[tauri::command]
pub async fn scan_snapshot() -> Result<Snapshot, String> {
    crate::scanner::snapshot::build_snapshot()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn upload_snapshot(
    user_id: String,
    device_name: String,
    hostname: String,
    operating_system: String,
    access_token: String,
) -> Result<String, String> {
    // Ensure this device exists in the database
    let device_id = ensure_device(
        &user_id,
        &device_name,
        &hostname,
        &operating_system,
        &access_token,
    )
    .await
    .map_err(|e| e.to_string())?;
    println!("===== upload_snapshot START =====");

println!("user_id: {}", user_id);
println!("device_id: {}", device_id);
println!("device_name: {}", device_name);
println!("access token length: {}", access_token.len());

println!("Uploading...");

    // Build snapshot
    let snapshot = crate::scanner::snapshot::build_snapshot()
        .await
        .map_err(|e| e.to_string())?;

    // Upload snapshot
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