use crate::models::device::Device;
use crate::models::snapshot::Snapshot;

use anyhow::{anyhow, Result};
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;

use crate::config::{SUPABASE_ANON_KEY, SUPABASE_BUCKET, SUPABASE_URL};

#[derive(Debug, Deserialize)]
struct DeviceRow {
    id: String,
    user_id: String,
    name: String,
}

fn sanitize_storage_component(value: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;

    for ch in value.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
            continue;
        }

        if (ch.is_whitespace() || ch == '-' || ch == '_') && !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }

    let out = out.trim_matches('-');
    if out.is_empty() {
        "device".to_string()
    } else {
        out.to_string()
    }
}

fn encode_storage_path(path: &str) -> String {
    fn encode_segment(segment: &str) -> String {
        let mut encoded = String::new();

        for byte in segment.as_bytes() {
            match *byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    encoded.push(*byte as char);
                }
                _ => encoded.push_str(&format!("%{:02X}", byte)),
            }
        }

        encoded
    }

    path.split('/')
        .map(encode_segment)
        .collect::<Vec<_>>()
        .join("/")
}
async fn download_snapshot_bytes(
    client: &Client,
    supabase_url: &str,
    anon_key: &str,
    access_token: &str,
    bucket: &str,
    storage_path: &str,
) -> Result<Vec<u8>> {
    let storage_path = storage_path.trim_start_matches('/');

    let encoded_path = encode_storage_path(storage_path);

    let storage_url = format!(
        "{}/storage/v1/object/authenticated/{}/{}",
        supabase_url.trim_end_matches('/'),
        bucket,
        encoded_path
    );

    println!("===== STORAGE DOWNLOAD =====");
    println!("Bucket: {}", bucket);
    println!("Path: {}", storage_path);
    println!("Encoded path: {}", encoded_path);
    println!("URL: {}", storage_url);
    println!("Access token length: {}", access_token.len());
    println!("============================");

    let response = client
        .get(&storage_url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let status = response.status();
    let body = response.bytes().await?;

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(&body);

        return Err(anyhow!(
            "Snapshot download failed: bucket={}, path={} => {} {}",
            bucket,
            storage_path,
            status,
            body_text
        ));
    }

    Ok(body.to_vec())
}

pub async fn upload_snapshot(
    user_id: &str,
    device_id: &str,
    device_name: &str,
    access_token: &str,
    snapshot: &Snapshot,
) -> Result<String> {
    dotenvy::dotenv().ok();

    let supabase_url = SUPABASE_URL;
    let anon_key = SUPABASE_ANON_KEY;

    let client = Client::new();

    let json = serde_json::to_vec(snapshot)?;
    let size = json.len();

    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let safe_device_name = sanitize_storage_component(device_name);
    let filename = format!("{}/{}/{}.json", user_id, safe_device_name, timestamp);

    let storage_url = format!("{}/storage/v1/object/snapshots/{}", supabase_url, filename);

    let response = client
        .post(storage_url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
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

        return Err(anyhow!("Upload failed: {} {}", status, body));
    }

    let db_url = format!("{}/rest/v1/snapshots", supabase_url);

    let response = client
        .post(db_url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
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

pub async fn fetch_devices(user_id: &str, access_token: &str) -> Result<Vec<Device>> {
    let supabase_url = SUPABASE_URL;
    let anon_key = SUPABASE_ANON_KEY;
    let client = reqwest::Client::new();

    let url = format!(
        "{}/rest/v1/devices?user_id=eq.{}&select=id,user_id,name",
        supabase_url, user_id
    );

    let response = client
        .get(url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(anyhow!("Devices fetch failed: {} {}", status, body));
    }

    let rows: Vec<DeviceRow> = serde_json::from_str(&body)
        .map_err(|e| anyhow!("Devices response decode failed: {} ({})", e, body))?;

    let mut devices = Vec::with_capacity(rows.len());

    for row in rows {
        let snapshot = fetch_latest_snapshot_for_device(&row.id, access_token)
            .await
            .ok();

        if let Some(snapshot) = snapshot {
            devices.push(Device {
                id: row.id,
                user_id: row.user_id,
                device_name: row.name,
                hostname: snapshot.system.hostname,
                os: snapshot.system.os,
                os_version: snapshot.system.os_version,
                cpu: snapshot.system.cpu,
                ram_gb: snapshot.system.ram_gb,
                disk_gb: snapshot.system.disk_gb,
            });
        } else {
            devices.push(Device {
                id: row.id,
                user_id: row.user_id,
                device_name: row.name,
                hostname: "-".to_string(),
                os: "-".to_string(),
                os_version: "-".to_string(),
                cpu: "-".to_string(),
                ram_gb: 0,
                disk_gb: 0,
            });
        }
    }

    Ok(devices)
}

pub async fn fetch_snapshot(snapshot_id: &str, access_token: &str) -> Result<Snapshot> {
    let supabase_url = SUPABASE_URL;
    let anon_key = SUPABASE_ANON_KEY;
    let bucket = SUPABASE_BUCKET;

    let client = reqwest::Client::new();

    let metadata_url = format!(
        "{}/rest/v1/snapshots?id=eq.{}&select=id,device_id,storage_path,created_at",
        supabase_url, snapshot_id
    );

    let response = client
        .get(metadata_url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .error_for_status()?;

    let rows: Vec<serde_json::Value> = response.json().await?;

    let storage_path = rows
        .first()
        .and_then(|row| row["storage_path"].as_str())
        .ok_or_else(|| anyhow!("Snapshot not found"))?;

    let bytes = download_snapshot_bytes(
        &client,
        supabase_url,
        anon_key,
        access_token,
        bucket,
        storage_path,
    )
    .await?;

    let snapshot: Snapshot = serde_json::from_slice(&bytes)?;

    Ok(snapshot)
}
pub async fn fetch_latest_snapshot_for_device(
    device_id: &str,
    access_token: &str,
) -> Result<Snapshot> {
    let supabase_url = SUPABASE_URL;
    let anon_key = SUPABASE_ANON_KEY;
    let bucket = SUPABASE_BUCKET;

    let client = reqwest::Client::new();

    let metadata_url = format!(
        "{}/rest/v1/snapshots?device_id=eq.{}&select=id,device_id,storage_path,created_at&order=created_at.desc&limit=20",
        supabase_url,
        device_id
    );

    let response = client
        .get(metadata_url)
        .header("apikey", anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(anyhow!(
            "Snapshot metadata request failed: {} {}",
            status,
            body
        ));
    }

    let rows: Vec<serde_json::Value> = serde_json::from_str(&body)?;

    println!("===== SNAPSHOT ROWS =====");

    for row in &rows {
        println!(
            "id={} | device_id={} | storage_path={} | created_at={}",
            row["id"].as_str().unwrap_or("-"),
            row["device_id"].as_str().unwrap_or("-"),
            row["storage_path"].as_str().unwrap_or("-"),
            row["created_at"].as_str().unwrap_or("-"),
        );
    }

    println!("=========================");

    let mut last_error: Option<String> = None;

    for row in rows {
        let Some(storage_path) = row["storage_path"].as_str() else {
            continue;
        };

        println!("Trying snapshot: {}", storage_path);

        match download_snapshot_bytes(
            &client,
            supabase_url,
            anon_key,
            access_token,
            bucket,
            storage_path,
        )
        .await
        {
            Ok(bytes) => {
                let snapshot: Snapshot = serde_json::from_slice(&bytes)?;

                println!("Successfully downloaded snapshot: {}", storage_path);

                return Ok(snapshot);
            }

            Err(err) => {
                eprintln!("Failed to download snapshot {}: {}", storage_path, err);

                last_error = Some(err.to_string());
            }
        }
    }

    Err(anyhow!(
        "No valid snapshot found for device {}. Last error: {}",
        device_id,
        last_error.unwrap_or_else(|| "no snapshots found".to_string())
    ))
}
