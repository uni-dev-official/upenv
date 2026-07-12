//! Authentication commands — delegate to `services::supabase_auth`.
//! Implemented fully in Milestone 2.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: String,
    pub email: String,
    pub access_token: String,
}

#[tauri::command]
pub async fn login(email: String, password: String) -> Result<AuthResponse, String> {
    crate::services::supabase_auth::sign_in(&email, &password)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn register(email: String, password: String) -> Result<AuthResponse, String> {
    crate::services::supabase_auth::sign_up(&email, &password)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    crate::services::supabase_auth::sign_out()
        .await
        .map_err(|e| e.to_string())
}
