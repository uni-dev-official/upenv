//! Collects Node.js version, npm version,
//! and globally installed npm packages.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeInfo {
    pub node_version: Option<String>,
    pub npm_version: Option<String>,
    pub global_packages: Vec<String>,
}

fn run_command(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| anyhow!("Failed to run {}: {}", command, e))?;

    if !output.status.success() {
        return Err(anyhow!(
            "{} failed: {}",
            command,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn scan_node() -> Result<NodeInfo> {
    let node_version = match run_command("node", &["-v"]) {
        Ok(version) => Some(version.trim().to_string()),
        Err(_) => None,
    };

    let npm_version = match run_command("npm", &["-v"]) {
        Ok(version) => Some(version.trim().to_string()),
        Err(_) => None,
    };

    let mut global_packages = Vec::new();

    if node_version.is_some() && npm_version.is_some() {
        if let Ok(output) = run_command("npm", &["list", "-g", "--depth=0", "--json"]) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                if let Some(dependencies) = json["dependencies"].as_object() {
                    global_packages.extend(dependencies.keys().cloned());
                }
            }
        }
    }

    Ok(NodeInfo {
        node_version,
        npm_version,
        global_packages,
    })
}
