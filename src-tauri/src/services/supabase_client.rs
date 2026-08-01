//! Thin Supabase REST/Auth client wrapper. Reads project URL and anon key
//! from environment variables set at build/runtime — never hardcoded.

use anyhow::{Context, Result};
use crate::config::{
    SUPABASE_ANON_KEY,
    SUPABASE_BUCKET,
    SUPABASE_URL,
};

pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
}

impl SupabaseConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: SUPABASE_URL.to_string(),
            anon_key: SUPABASE_ANON_KEY.to_string(),
        })
    }
}

pub fn client() -> reqwest::Client {
    reqwest::Client::new()
}
