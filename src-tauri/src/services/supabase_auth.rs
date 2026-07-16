use crate::commands::auth::AuthResponse;
use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SupabaseSignupResponse {
    id: Option<String>,
    email: Option<String>,
}

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
    let supabase_url =
        std::env::var("SUPABASE_URL")
            .map_err(|_| anyhow!("Missing SUPABASE_URL"))?;

    let anon_key =
        std::env::var("SUPABASE_ANON_KEY")
            .map_err(|_| anyhow!("Missing SUPABASE_ANON_KEY"))?;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/auth/v1/signup", supabase_url))
        .header("apikey", &anon_key)
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

    let user: SupabaseSignupResponse =
        serde_json::from_str(&body)?;

    let login = sign_in(email, password).await?;

    Ok(login)
}


pub async fn sign_in(email: &str, password: &str) -> Result<AuthResponse> {
    let supabase_url =
        std::env::var("SUPABASE_URL")?;

    let anon_key =
        std::env::var("SUPABASE_ANON_KEY")?;

    let client = reqwest::Client::new();

    let response = client
        .post(format!(
            "{}/auth/v1/token?grant_type=password",
            supabase_url
        ))
        .header("apikey", &anon_key)
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
        return Err(anyhow!("Supabase login failed: {}", body));
    }

    let token: SupabaseTokenResponse =
        serde_json::from_str(&body)?;

    Ok(AuthResponse {
        user_id: token.user.id,
        email: token.user.email,
        access_token: token.access_token,
    })
}


pub async fn sign_out() -> Result<()> {
    Ok(())
}