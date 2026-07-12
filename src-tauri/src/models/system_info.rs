use serde::{Deserialize, Serialize};

/// Basic hardware/OS metadata collected during a scan.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemInfo {
    pub device_name: String,
    pub os: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu: String,
    pub ram_gb: u32,
    pub disk_gb: u32,
}
