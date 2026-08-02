//! Collects basic system information: device name, OS, CPU, RAM, disk, hostname.

use crate::models::system_info::SystemInfo;
use anyhow::Result;
use std::process::Command;

pub async fn collect_system_info() -> Result<SystemInfo> {
    let hostname = get_hostname()?;
    let device_name = get_device_name()?;

    Ok(SystemInfo {
        device_name,
        os: "macOS".to_string(),
        os_version: get_os_version()?,
        hostname,
        cpu: get_cpu()?,
        ram_gb: get_ram()?,
        disk_gb: get_disk()?,
    })
}

fn run_command(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command).args(args).output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_hostname() -> Result<String> {
    run_command("hostname", &[])
}

fn get_device_name() -> Result<String> {
    run_command("scutil", &["--get", "ComputerName"])
}

fn get_os_version() -> Result<String> {
    run_command("sw_vers", &["-productVersion"])
}

fn get_cpu() -> Result<String> {
    run_command("sysctl", &["-n", "machdep.cpu.brand_string"])
}

fn get_ram() -> Result<u32> {
    let bytes = run_command("sysctl", &["-n", "hw.memsize"])?.parse::<u64>()?;

    Ok((bytes / 1024 / 1024 / 1024) as u32)
}

fn get_disk() -> Result<u32> {
    let output = run_command("df", &["-k", "/"])?;

    let lines: Vec<&str> = output.lines().collect();

    if lines.len() < 2 {
        return Ok(0);
    }

    let parts: Vec<&str> = lines[1].split_whitespace().collect();

    if parts.len() < 2 {
        return Ok(0);
    }

    let kb: u64 = parts[1].parse()?;

    Ok((kb / 1024 / 1024) as u32)
}
