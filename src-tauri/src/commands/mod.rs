//! Tauri commands: the only layer the frontend talks to.
//! Commands stay thin — they parse input, delegate to scanner/restore/services,
//! and map results into Tauri-friendly Result<T, String> responses.

pub mod auth;
pub mod devices;
pub mod restore;
pub mod scan;
pub mod snapshot;
pub mod system;
