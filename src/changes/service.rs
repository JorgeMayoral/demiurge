use std::fmt::Display;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use crate::config::{Service, Services};

#[derive(Debug, Clone)]
pub struct ServiceChanges {
    pub enable: Services,
    pub disable: Services,
}

impl ServiceChanges {
    pub fn new(new_services_config: &Services, applied_services_config: &Services) -> Self {
        let services_to_enable: Vec<Service> = new_services_config
            .services()
            .iter()
            .filter(|service| !applied_services_config.services().contains(service))
            .map(ToOwned::to_owned)
            .collect();
        let services_to_disable: Vec<Service> = applied_services_config
            .services()
            .iter()
            .filter(|service| !new_services_config.services().contains(service))
            .map(ToOwned::to_owned)
            .collect();

        Self {
            enable: Services::new(services_to_enable),
            disable: Services::new(services_to_disable),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.enable.services().is_empty() && self.disable.services().is_empty()
    }

    pub fn apply(&self) -> Result<()> {
        for service in self.enable.services() {
            service
                .enable()
                .with_context(|| format!("enable service {}", service.service()))?;
        }

        for service in self.disable.services() {
            service
                .disable()
                .with_context(|| format!("disable service {}", service.service()))?;
        }

        Ok(())
    }
}

impl Display for ServiceChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "Services".blue().bold().to_string();
        let services_to_enable = self
            .enable
            .services()
            .iter()
            .map(|service| {
                let symbol = "+".green().to_string();
                format!("[{}] {}", symbol, service.service())
            })
            .collect::<Vec<String>>();
        let services_to_enable_text = if services_to_enable.is_empty() {
            "No services to enable".yellow().to_string()
        } else {
            services_to_enable.join("\n")
        };

        let services_to_disable = self
            .disable
            .services()
            .iter()
            .map(|service| {
                let symbol = "-".red().to_string();
                format!("[{}] {}", symbol, service.service())
            })
            .collect::<Vec<String>>();
        let services_to_disable_text = if services_to_disable.is_empty() {
            "No services to disable".yellow().to_string()
        } else {
            services_to_disable.join("\n")
        };

        let text = format!("{title}\n{services_to_enable_text}\n\n{services_to_disable_text}");
        write!(f, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceChanges;
    use crate::config::Services;

    fn svcs(json: &str) -> Services {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn apply_does_nothing_when_no_changes() {
        let svcs_config = svcs(r#"["nginx", "docker"]"#);
        let changes = ServiceChanges::new(&svcs_config, &svcs_config);
        assert!(changes.apply().is_ok());
    }

    #[test]
    fn new_service_goes_to_enable_set() {
        let new = svcs(r#"["nginx", "docker"]"#);
        let applied = svcs(r#"["nginx"]"#);
        let changes = ServiceChanges::new(&new, &applied);
        let enable_names: Vec<String> = changes
            .enable
            .services()
            .iter()
            .map(|s| s.service())
            .collect();
        assert!(enable_names.contains(&"docker".to_owned()));
        assert!(changes.disable.services().is_empty());
    }

    #[test]
    fn removed_service_goes_to_disable_set() {
        let new = svcs(r#"["nginx"]"#);
        let applied = svcs(r#"["nginx", "docker"]"#);
        let changes = ServiceChanges::new(&new, &applied);
        let disable_names: Vec<String> = changes
            .disable
            .services()
            .iter()
            .map(|s| s.service())
            .collect();
        assert!(disable_names.contains(&"docker".to_owned()));
        assert!(changes.enable.services().is_empty());
    }

    #[test]
    fn unchanged_services_produce_empty_sets() {
        let svcs_config = svcs(r#"["nginx", "docker"]"#);
        let changes = ServiceChanges::new(&svcs_config, &svcs_config);
        assert!(changes.enable.services().is_empty());
        assert!(changes.disable.services().is_empty());
    }
}
