use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A registered device belonging to a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// UUID of this device record.
    pub id: String,

    /// Supabase user id.
    pub user_id: String,

    /// Human-readable device name (e.g. "MacBook Pro").
    pub device_name: String,

    /// Computer hostname.
    pub hostname: String,

    /// Operating system.
    pub os: String,

    /// OS version.
    pub os_version: String,

    /// CPU model.
    pub cpu: String,

    /// RAM size (GB).
    pub ram_gb: u32,

    /// Disk size (GB).
    pub disk_gb: u32,

    /// Time of the latest successful scan.
    pub last_backup: DateTime<Utc>,

    /// UUID of the latest uploaded snapshot.
    pub snapshot_id: String,
}