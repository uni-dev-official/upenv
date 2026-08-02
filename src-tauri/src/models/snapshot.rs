use crate::models::system_info::SystemInfo;
use crate::scanner::docker::DockerInfo;
use crate::scanner::git::GitInfo;
use crate::scanner::node::NodeInfo;
use crate::scanner::python::PythonInfo;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub system: SystemInfo,

    pub applications: Vec<String>,
    pub brew_packages: Vec<String>,
    pub brew_casks: Vec<String>,
    pub vscode_extensions: Vec<String>,

    pub git: GitInfo,
    pub node: NodeInfo,
    pub python: PythonInfo,
    pub docker: DockerInfo,

    pub configs: Vec<String>,

    pub created_at: DateTime<Utc>,
}
