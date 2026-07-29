//! Installs applications where possible via Homebrew casks.
//! Reports per-application progress to the restore pipeline.

use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::scanner::applications::scan_applications;
use crate::utils::shell;

#[derive(Debug, Default)]
pub struct AppRestoreSummary {
    pub installed: usize,
    pub skipped: usize,
}

fn normalize_app_name(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace(' ', "")
}

fn app_to_brew_cask(name: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;

    for ch in name.trim().to_ascii_lowercase().chars() {
        let mapped = match ch {
            'a'..='z' | '0'..='9' => Some(ch),
            '&' => Some('-'),
            '+' => Some('-'),
            ' ' | '_' | '.' | '/' | '(' | ')' => Some('-'),
            _ => None,
        };

        if let Some(c) = mapped {
            if c == '-' {
                if !last_dash && !slug.is_empty() {
                    slug.push('-');
                    last_dash = true;
                }
            } else {
                slug.push(c);
                last_dash = false;
            }
        }
    }

    slug.trim_matches('-').to_string()
}

pub async fn install_applications<F>(
    snapshot_apps: &[String],
    mut progress: F,
) -> Result<AppRestoreSummary>
where
    F: FnMut(usize, usize, &str, &str),
{
    if snapshot_apps.is_empty() {
        return Ok(AppRestoreSummary::default());
    }

    if !shell::command_exists("brew").await {
        anyhow::bail!("Homebrew is required to restore applications");
    }

    let installed_apps = scan_applications().await?;

    let installed_set: HashSet<String> = installed_apps
        .iter()
        .map(|name| normalize_app_name(name))
        .collect();

    let mut seen = HashSet::new();
    let mut summary = AppRestoreSummary::default();

    let total = snapshot_apps.len();
    let mut current = 0;

    for app_name in snapshot_apps {
        let normalized = normalize_app_name(app_name);

        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }

        current += 1;

        // Already installed
        if installed_set.contains(&normalized) {
            progress(
                current,
                total,
                app_name,
                "already installed",
            );

            summary.skipped += 1;
            continue;
        }

        // Convert application name to Homebrew cask
        let cask = app_to_brew_cask(app_name);

        if cask.is_empty() {
            progress(
                current,
                total,
                app_name,
                "failed",
            );

            anyhow::bail!(
                "Cannot map application `{}` to a brew cask name",
                app_name
            );
        }

        // Tell frontend installation has started
        progress(
            current,
            total,
            app_name,
            "installing",
        );

        // Install application
        match shell::run(
            "brew",
            &["install", "--cask", &cask],
        )
        .await
        {
            Ok(_) => {
                progress(
                    current,
                    total,
                    app_name,
                    "installed",
                );

                summary.installed += 1;
            }

            Err(error) => {
                progress(
                    current,
                    total,
                    app_name,
                    "failed",
                );

                return Err(error).with_context(|| {
                    format!(
                        "Failed to install missing app `{}` (cask `{}`)",
                        app_name, cask
                    )
                });
            }
        }
    }

    Ok(summary)
}