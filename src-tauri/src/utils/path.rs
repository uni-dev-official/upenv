//! Utilities for making external command-line tools available
//! when Restorely is launched as a macOS GUI application.
//
// macOS GUI applications launched from Finder may receive a much
// smaller PATH than applications launched from Terminal.

use std::env;

const MACOS_PATHS: &[&str] = &[
    "/usr/local/bin",
    "/opt/homebrew/bin",
    "/usr/bin",
    "/bin",
    "/usr/sbin",
    "/sbin",
];

/// Adds common macOS command locations to the current process PATH.
pub fn initialize_path() {
    let current_path = env::var("PATH").unwrap_or_default();

    let mut paths: Vec<String> = MACOS_PATHS.iter().map(|path| path.to_string()).collect();

    for path in env::split_paths(&current_path) {
        let path = path.to_string_lossy().to_string();

        if !paths.contains(&path) {
            paths.push(path);
        }
    }

    if let Ok(new_path) = env::join_paths(paths) {
        env::set_var("PATH", new_path);
    }

    println!("Restorely PATH initialized:");
    println!("{}", env::var("PATH").unwrap_or_default());
}
