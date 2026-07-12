// Prevents additional console window on Windows in release, DO NOT REMOVE (unused on macOS but harmless)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    dotenvy::dotenv().ok();
    restorely_lib::run();
}
