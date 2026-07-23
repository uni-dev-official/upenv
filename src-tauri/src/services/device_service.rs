use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub user_id: String,
    pub name: String,
}

async fn decode_devices_response(body: &str, context: &str) -> Result<Vec<Device>> {
    serde_json::from_str(body)
        .map_err(|e| anyhow!("{context} response decode failed: {e} ({body})"))
}

pub async fn ensure_device(
    user_id: &str,
    device_name: &str,
    _hostname: &str,
    _operating_system: &str,
    access_token: &str,
) -> Result<String> {
    let client = Client::new();

    let supabase_url = std::env::var("SUPABASE_URL")
        .map_err(|_| anyhow!("Missing SUPABASE_URL"))?;

    let anon_key = std::env::var("SUPABASE_ANON_KEY")
        .map_err(|_| anyhow!("Missing SUPABASE_ANON_KEY"))?;

    let lookup_response = client
        .get(format!("{}/rest/v1/devices", supabase_url))
        .query(&[
            ("user_id", format!("eq.{}", user_id)),
            ("name", format!("eq.{}", device_name)),
        ])
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let lookup_status = lookup_response.status();
    let lookup_body = lookup_response.text().await?;

    if !lookup_status.is_success() {
        return Err(anyhow!(
            "Device lookup failed: {} {}",
            lookup_status,
            lookup_body
        ));
    }

    let devices = decode_devices_response(&lookup_body, "Device lookup").await?;
    if let Some(device) = devices.first() {
        println!("Existing device found: {}", device.id);
        return Ok(device.id.clone());
    }

    let new_device = json!({
        "user_id": user_id,
        "name": device_name
    });

    let create_response = client
        .post(format!("{}/rest/v1/devices", supabase_url))
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Prefer", "return=representation")
        .json(&new_device)
        .send()
        .await?;

    let create_status = create_response.status();
    let create_body = create_response.text().await?;

    if !create_status.is_success() {
        return Err(anyhow!(
            "Device creation failed: {} {}",
            create_status,
            create_body
        ));
    }

    match decode_devices_response(&create_body, "Device creation").await {
        Ok(created) => {
            let device = created
                .first()
                .ok_or_else(|| anyhow!("Device creation failed: empty response"))?;
            println!("Created new device: {}", device.id);
            Ok(device.id.clone())
        }
        Err(_) => {
            let refetch_response = client
                .get(format!("{}/rest/v1/devices", supabase_url))
                .query(&[
                    ("user_id", format!("eq.{}", user_id)),
                    ("name", format!("eq.{}", device_name)),
                ])
                .header("apikey", &anon_key)
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
                .await?;

            let refetch_status = refetch_response.status();
            let refetch_body = refetch_response.text().await?;

            if !refetch_status.is_success() {
                return Err(anyhow!(
                    "Device refetch failed: {} {}",
                    refetch_status,
                    refetch_body
                ));
            }

            let created = decode_devices_response(&refetch_body, "Device refetch").await?;
            let device = created
                .first()
                .ok_or_else(|| anyhow!("Device creation failed: no device found after insert"))?;
            println!("Created new device: {}", device.id);
            Ok(device.id.clone())
        }
    }
}
