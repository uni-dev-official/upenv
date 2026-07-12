//! Thin Supabase REST/Auth client wrapper. Reads project URL and anon key
//! from environment variables set at build/runtime — never hardcoded.

use anyhow::{Context, Result};

pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
}

impl SupabaseConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: std::env::var("SUPABASE_URL").context("SUPABASE_URL not set")?,
            anon_key: std::env::var("SUPABASE_ANON_KEY").context("SUPABASE_ANON_KEY not set")?,
        })
    }
}

pub fn client() -> reqwest::Client {
    reqwest::Client::new()
}
