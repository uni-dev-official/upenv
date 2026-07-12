//! Uploads/downloads snapshots and fetches device records from Supabase
//! PostgreSQL + Storage. Implemented in Milestones 6-7.

use crate::models::device::Device;
use crate::models::snapshot::Snapshot;
use anyhow::Result;


pub async fn upload_snapshot(
    user_id: &str,
    device_name: &str,
    snapshot: &Snapshot,
) -> Result<String> {

    println!("=== Snapshot upload started ===");
    println!("User ID: {}", user_id);
    println!("Device name: {}", device_name);

    println!(
        "Snapshot data: {:?}",
        snapshot
    );


    // TODO Milestone 6:
    // 1. Convert snapshot into JSON
    // 2. Upload machine.json to Supabase Storage
    // 3. Insert metadata into snapshots table
    // 4. Return generated snapshot ID


    println!("=== Snapshot upload completed ===");


    // Temporary test ID
    // This confirms that Tauri -> Rust -> service works.
    Ok("test-upload-id".to_string())
}


pub async fn fetch_devices(
    user_id: &str,
) -> Result<Vec<Device>> {

    println!(
        "Fetching devices for user: {}",
        user_id
    );


    // TODO Milestone 7:
    // SELECT *
    // FROM devices
    // WHERE user_id = $1


    Ok(Vec::new())
}


pub async fn fetch_snapshot(
    snapshot_id: &str,
) -> Result<Snapshot> {

    println!(
        "Fetching snapshot: {}",
        snapshot_id
    );


    // TODO Milestone 8:
    // 1. Find snapshot record
    // 2. Download machine.json from Supabase Storage
    // 3. Deserialize JSON into Snapshot


    anyhow::bail!("fetch_snapshot not implemented yet")
}