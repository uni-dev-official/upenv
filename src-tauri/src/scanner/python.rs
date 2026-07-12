//! Collects Python version, pip version, and installed packages.
//! Implemented in Milestone 4.

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PythonInfo {
    pub python_version: Option<String>,
    pub pip_version: Option<String>,
    pub packages: Vec<String>,
}

pub async fn scan_python() -> Result<PythonInfo> {
    // Milestone 4: `python3 --version`, `pip3 --version`, `pip3 list`.
    Ok(PythonInfo::default())
}
