use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub async fn scan_applications() -> Result<Vec<String>> {
    let mut apps = Vec::new();

    let paths = vec![
        PathBuf::from("/Applications"),
        dirs::home_dir().unwrap_or_default().join("Applications"),
    ];

    for path in paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Some(name) = path.file_name() {
                    let name = name.to_string_lossy();

                    if name.ends_with(".app") {
                        apps.push(name.trim_end_matches(".app").to_string());
                    }
                }
            }
        }
    }

    Ok(apps)
}
