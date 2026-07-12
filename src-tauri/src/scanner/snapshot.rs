//! Aggregates all individual scanners into a single `Snapshot`.
//! This is the orchestration point for Milestone 5.

use crate::models::snapshot::Snapshot;
use anyhow::Result;

pub async fn build_snapshot() -> Result<Snapshot> {
    let system = super::system::collect_system_info().await?;
    let applications = super::applications::scan_applications().await?;
    let brew = super::brew::scan_brew().await?;
    let git = super::git::scan_git().await?;
    let vscode = super::vscode::scan_vscode().await?;
    let node = super::node::scan_node().await?;
    let python = super::python::scan_python().await?;
    let docker = super::docker::scan_docker().await?;

    Ok(Snapshot {
        system,
        applications,
        brew_packages: brew.packages,
        brew_casks: brew.casks,
        vscode_extensions: vscode.extensions,
        git,
        node,
        python,
        docker,
        configs: Vec::new(),
        created_at: chrono::Utc::now(),
    })
}
