use crate::commands::auth::AuthResponse;
use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::config::{SUPABASE_ANON_KEY, SUPABASE_URL};

#[derive(Debug, Deserialize)]
struct SupabaseSignupResponse {}

#[derive(Debug, Deserialize)]
struct SupabaseTokenResponse {
    access_token: String,
    user: SupabaseUser,
}

#[derive(Debug, Deserialize)]
struct SupabaseUser {
    id: String,
    email: String,
}

pub async fn sign_up(email: &str, password: &str) -> Result<AuthResponse> {
    let supabase_url = SUPABASE_URL;

    let anon_key = SUPABASE_ANON_KEY;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/auth/v1/signup", supabase_url))
        .header("apikey", anon_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;

    let status = response.status();

    let body = response.text().await?;

    if !status.is_success() {
        return Err(anyhow!("Supabase signup failed: {}", body));
    }

    let _: SupabaseSignupResponse = serde_json::from_str(&body)?;

    let login = sign_in(email, password).await?;

    Ok(login)
}

pub async fn sign_in(email: &str, password: &str) -> Result<AuthResponse> {
    let supabase_url = SUPABASE_URL;

    let anon_key = SUPABASE_ANON_KEY;

    let client = reqwest::Client::new();

    let response = client
        .post(format!(
            "{}/auth/v1/token?grant_type=password",
            supabase_url
        ))
        .header("apikey", anon_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;

    let status = response.status();

    let body = response.text().await?;

    if !status.is_success() {
        return Err(anyhow!("Password or email is incorrect"));
    }

    let token: SupabaseTokenResponse = serde_json::from_str(&body)?;

    Ok(AuthResponse {
        user_id: token.user.id,
        email: token.user.email,
        access_token: token.access_token,
    })
}

pub async fn sign_out() -> Result<()> {
    Ok(())
}
