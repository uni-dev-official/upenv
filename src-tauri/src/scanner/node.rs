//! Collects Node.js version, npm version, and global npm packages.
//! Implemented in Milestone 4.

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeInfo {
    pub node_version: Option<String>,
    pub npm_version: Option<String>,
    pub global_packages: Vec<String>,
}

pub async fn scan_node() -> Result<NodeInfo> {
    // Milestone 4: `node -v`, `npm -v`, `npm list -g --depth=0`.
    Ok(NodeInfo::default())
}
