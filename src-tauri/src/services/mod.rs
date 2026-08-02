//! Cross-cutting services: external integrations (Supabase auth/storage).
//! Commands call into these; scanner/restore never talk to the network
//! directly, keeping side effects isolated to this layer.

pub mod device_service;
pub mod supabase_auth;
pub mod supabase_client;
pub mod supabase_storage;
