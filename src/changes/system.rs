use std::fmt::Display;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::config::System;

#[derive(Debug)]
pub struct SystemChanges {
    pub hostname: Option<String>,
}

impl SystemChanges {
    pub fn new(new_system_config: &System, applied_system_config: &System) -> Self {
        let hostname = if new_system_config.hostname() == applied_system_config.hostname() {
            None
        } else {
            Some(new_system_config.hostname())
        };
        Self { hostname }
    }

    pub fn apply(&self) -> Result<()> {
        if let Some(hostname) = self.hostname.clone() {
            let current_hostname = duct::cmd!("hostname")
                .read()
                .context("read current hostname")?;

            if hostname != current_hostname {
                log::info!("Changing hostname from {current_hostname} to {hostname}");
                duct::cmd!("sudo", "hostname", &hostname)
                    .run()
                    .with_context(|| format!("set hostname to {hostname}"))?;
            }
        } else {
            log::info!("No hostname change.");
        }

        Ok(())
    }
}

impl Display for SystemChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "System".blue().bold().to_string();
        let hostname_change = match self.hostname.clone() {
            Some(hostname) => hostname.green().to_string(),
            None => "Unchanged".yellow().to_string(),
        };
        let text = format!("{title}\n{hostname_change}");
        write!(f, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use super::SystemChanges;
    use crate::config::System;

    fn make_system(hostname: &str) -> System {
        serde_json::from_value(serde_json::json!({ "hostname": hostname })).unwrap()
    }

    #[test]
    fn hostname_unchanged_produces_no_change() {
        let system = make_system("myhost");
        let changes = SystemChanges::new(&system, &system);
        assert!(changes.hostname.is_none());
    }

    #[test]
    fn hostname_changed_captures_new_value() {
        let new_system = make_system("newhost");
        let applied_system = make_system("oldhost");
        let changes = SystemChanges::new(&new_system, &applied_system);
        assert_eq!(changes.hostname, Some("newhost".to_owned()));
    }
}
