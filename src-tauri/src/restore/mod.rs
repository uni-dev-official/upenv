//! Restore: appliers that reconstruct a machine from a Snapshot.
//! Each file owns exactly one restoration concern and reports progress
//! via Tauri events so the frontend can render a live log.

pub mod restore;
pub mod restore_apps;
pub mod restore_brew;
pub mod restore_git;
pub mod restore_node;
pub mod restore_python;
pub mod restore_vscode;
