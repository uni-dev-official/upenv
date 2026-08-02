//! Collects Docker version and Docker Compose version.
//! Implemented in Milestone 4.

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DockerInfo {
    pub docker_version: Option<String>,
    pub compose_version: Option<String>,
}

pub async fn scan_docker() -> Result<DockerInfo> {
    // Milestone 4: `docker --version`, `docker compose version`.
    Ok(DockerInfo::default())
}
