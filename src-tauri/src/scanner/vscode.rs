//! Collects VS Code installed extensions.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeInfo {
    pub extensions: Vec<String>,
}

pub async fn scan_vscode() -> Result<VSCodeInfo> {
    let output = Command::new("code").args(["--list-extensions"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let extensions = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|s| s.to_string())
                .collect();

            Ok(VSCodeInfo { extensions })
        }

        _ => Ok(VSCodeInfo {
            extensions: Vec::new(),
        }),
    }
}
