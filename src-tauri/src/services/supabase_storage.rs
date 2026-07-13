use crate::models::device::Device;
use crate::models::snapshot::Snapshot;

use anyhow::{anyhow, Result};
use reqwest::Client;
use std::env;
use uuid::Uuid;

pub async fn upload_snapshot(
    user_id: &str,
    device_name: &str,
    snapshot: &Snapshot,
) -> Result<String> {
    dotenvy::dotenv().ok();

    let supabase_url = env::var("SUPABASE_URL")?;
    let anon_key = env::var("SUPABASE_ANON_KEY")?;

    let client = Client::new();

    let json = serde_json::to_vec(snapshot)?;

    let filename = format!(
        "{}/{}/{}.json",
        user_id,
        device_name,
        Uuid::new_v4()
    );

    let url = format!(
        "{}/storage/v1/object/snapshots/{}",
        supabase_url,
        filename
    );

    let response = client
        .post(url)
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", anon_key))
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Upload failed: {}",
            response.text().await?
        ));
    }

    println!("Snapshot uploaded successfully!");

    Ok(filename)
}

pub async fn fetch_devices(
    _user_id: &str,
) -> Result<Vec<Device>> {
    Ok(Vec::new())
}

pub async fn fetch_snapshot(
    _snapshot_id: &str,
) -> Result<Snapshot> {
    anyhow::bail!("Not implemented")
}