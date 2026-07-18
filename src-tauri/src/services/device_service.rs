use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub user_id: String,
    pub device_name: String,
    pub os: String,
    pub os_version: String,
}


pub async fn ensure_device(
    user_id: &str,
    device_name: &str,
    os: &str,
    os_version: &str,
    access_token: &str,
) -> Result<String> {

    let client = Client::new();

    let supabase_url =
        std::env::var("SUPABASE_URL")
            .map_err(|_| anyhow!("Missing SUPABASE_URL"))?;

    let anon_key =
        std::env::var("SUPABASE_ANON_KEY")
            .map_err(|_| anyhow!("Missing SUPABASE_ANON_KEY"))?;


    //
    // 1. Search existing device
    //
    let response = client
        .get(format!(
            "{}/rest/v1/devices",
            supabase_url
        ))
        .query(&[
            ("user_id", format!("eq.{}", user_id)),
            ("device_name", format!("eq.{}", device_name)),
        ])
        .header("apikey", &anon_key)
        .header(
            "Authorization",
            format!("Bearer {}", access_token),
        )
        .send()
        .await?;


    let devices: Vec<Device> = response.json().await?;


    if let Some(device) = devices.first() {

        println!(
            "Existing device found: {}",
            device.id
        );

        return Ok(device.id.clone());
    }



    //
    // 2. Insert new device
    //
    let new_device = json!({
        "user_id": user_id,
        "device_name": device_name,
        "os": os,
        "os_version": os_version
    });


    let response = client
        .post(format!(
            "{}/rest/v1/devices",
            supabase_url
        ))
        .header("apikey", &anon_key)
        .header(
            "Authorization",
            format!("Bearer {}", access_token),
        )
        .header(
            "Prefer",
            "return=representation",
        )
        .json(&new_device)
        .send()
        .await?;


    let created: Vec<Device> = response.json().await?;


    let device = created
        .first()
        .ok_or(anyhow!("Device creation failed"))?;


    println!(
        "Created new device: {}",
        device.id
    );


    Ok(device.id.clone())
}