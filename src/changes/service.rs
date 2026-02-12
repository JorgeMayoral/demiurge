use std::fmt::Display;

use owo_colors::OwoColorize;

use crate::config::{Service, Services};

#[derive(Debug, Clone)]
pub struct ServiceChanges {
    pub enable: Services,
    pub disable: Services,
}

impl ServiceChanges {
    #[must_use]
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

    pub fn apply(&self) {
        self.enable
            .services()
            .iter()
            .for_each(|service| service.enable().unwrap());

        self.disable
            .services()
            .iter()
            .for_each(|service| service.disable().unwrap());
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
