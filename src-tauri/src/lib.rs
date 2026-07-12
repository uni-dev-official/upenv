//! Restorely core library.
//!
//! Modular layout:
//! - `commands`  : Tauri command handlers exposed to the frontend (thin glue layer)
//! - `scanner`   : Read-only collectors that inspect the local machine
//! - `restore`   : Installers/appliers that reconstruct a machine from a snapshot
//! - `models`    : Shared serde data structures (Snapshot, Device, etc.)
//! - `services`  : Cross-cutting services (Supabase client, auth, storage)
//! - `utils`     : Small stateless helpers (fs paths, shell exec, logging)

pub mod commands;
pub mod models;
pub mod restore;
pub mod scanner;
pub mod services;
pub mod utils;

/// Builds and runs the Tauri application.
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::system::scan_system,
            commands::auth::login,
            commands::auth::register,
            commands::auth::logout,
            commands::snapshot::scan_snapshot,
            commands::snapshot::upload_snapshot,
            commands::devices::list_devices,

        ])
        .run(tauri::generate_context!())
        .expect("error while running Restorely");
}
