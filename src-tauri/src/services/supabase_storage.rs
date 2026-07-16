use crate::models::device::Device;
use crate::models::snapshot::Snapshot;

use anyhow::{anyhow, Result};
use reqwest::Client;
use std::env;
use chrono::Utc;

pub async fn upload_snapshot(
    user_id: &str,
    device_id: &str,
    device_name: &str,
    access_token: &str,
    snapshot: &Snapshot,
) -> Result<String> {

    dotenvy::dotenv().ok();

    let supabase_url = env::var("SUPABASE_URL")?;
    let anon_key = env::var("SUPABASE_ANON_KEY")?;

    let client = Client::new();


    let json = serde_json::to_vec(snapshot)?;

    let size = json.len();


    let timestamp = Utc::now()
        .format("%Y-%m-%d_%H-%M-%S")
        .to_string();


    let filename = format!(
        "{}/{}/{}.json",
        user_id,
        device_name,
        timestamp
    );


    // Upload JSON file

    let storage_url = format!(
        "{}/storage/v1/object/snapshots/{}",
        supabase_url,
        filename
    );


    let response = client
        .post(storage_url)
        .header("apikey", &anon_key)
        .header(
            "Authorization",
            format!("Bearer {}", access_token)
        )
        .header(
            "Content-Type",
            "application/json"
        )
        .body(json)
        .send()
        .await?;


    if !response.status().is_success() {

        let status = response.status();

        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "No response body".to_string());

        eprintln!("SUPABASE STATUS: {}", status);
        eprintln!("SUPABASE RESPONSE: {}", body);

        return Err(anyhow!(
            "Upload failed: {} {}",
            status,
            body
        ));
    }



    // Insert metadata into snapshots table

    let db_url = format!(
        "{}/rest/v1/snapshots",
        supabase_url
    );


    let response = client
        .post(db_url)
        .header("apikey", &anon_key)
        .header(
            "Authorization",
            format!("Bearer {}", access_token)
        )
        .header(
            "Content-Type",
            "application/json"
        )
        .json(&serde_json::json!({
            "user_id": user_id,
            "device_id": device_id,
            "storage_path": filename,
            "size": size
        }))
        .send()
        .await?;


    if !response.status().is_success() {
        return Err(anyhow!(
            "Snapshot database insert failed: {}",
            response.text().await?
        ));
    }


    println!("Snapshot uploaded and registered!");

    Ok(filename)
}

pub async fn fetch_devices(
    _user_id: &str,
) -> Result<Vec<Device>> {
    Ok(Vec::new())
}

pub async fn fetch_snapshot(
    snapshot_id: &str,
    access_token: &str,
) -> Result<Snapshot> {

    let supabase_url = std::env::var("SUPABASE_URL")?;
    let anon_key = std::env::var("SUPABASE_ANON_KEY")?;

    let client = reqwest::Client::new();


    // 1. Get snapshot metadata from database
    let metadata_url = format!(
        "{}/rest/v1/snapshots?id=eq.{}&select=storage_path",
        supabase_url,
        snapshot_id
    );


    let response = client
        .get(metadata_url)
        .header("apikey", &anon_key)
        .header(
            "Authorization",
            format!("Bearer {}", access_token)
        )
        .send()
        .await?
        .error_for_status()?;


    let rows: Vec<serde_json::Value> =
        response.json().await?;


    let storage_path = rows
        .first()
        .and_then(|row| row["storage_path"].as_str())
        .ok_or_else(|| anyhow!("Snapshot not found"))?;



    // 2. Download JSON from storage

    let storage_url = format!(
        "{}/storage/v1/object/snapshots/{}",
        supabase_url,
        storage_path
    );


    let response = client
        .get(storage_url)
        .header("apikey", &anon_key)
        .header(
            "Authorization",
            format!("Bearer {}", access_token)
        )
        .send()
        .await?
        .error_for_status()?;


    let bytes = response.bytes().await?;


    // 3. Convert JSON into Snapshot struct

    let snapshot: Snapshot =
        serde_json::from_slice(&bytes)?;


    Ok(snapshot)
}