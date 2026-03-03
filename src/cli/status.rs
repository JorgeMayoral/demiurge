use clap::Args;
use owo_colors::OwoColorize;

use crate::config::{DemiurgeConfig, Service};

#[derive(Debug, Args, Clone)]
pub struct StatusArgs;

impl StatusArgs {
    pub fn run() {
        let Some(config) = DemiurgeConfig::read_applied_config() else {
            let msg = "No configuration has been applied yet."
                .yellow()
                .bold()
                .to_string();
            println!("{msg}");
            return;
        };

        let title = "Applied Configuration"
            .green()
            .bold()
            .underline()
            .to_string();
        println!("{title}");

        let system_header = "System".blue().bold().to_string();
        let hostname = config.system().hostname();
        let hostname_display = if hostname.is_empty() {
            "None".yellow().to_string()
        } else {
            format!("Hostname: {}", hostname.green())
        };
        println!("\n{system_header}\n{hostname_display}");

        let packages_header = "Packages".blue().bold().to_string();
        let mut package_managers = config.packages().package_managers();
        println!("\n{packages_header}");
        if package_managers.is_empty() {
            let msg = "None".yellow().to_string();
            println!("{msg}");
        } else {
            package_managers.sort();
            for pm in package_managers {
                let pkgs = config.packages().get(&pm).unwrap_or_default();
                println!("{}\n{}\n", pm.blue().underline(), pkgs.join("\n"));
            }
        }

        let dotfiles_header = "Dotfiles".blue().bold().to_string();
        let dotfiles = config.dotfiles().dotfiles();
        println!("\n{dotfiles_header}");
        if dotfiles.is_empty() {
            let msg = "None".yellow().to_string();
            println!("{msg}");
        } else {
            for dotfile in dotfiles {
                println!(
                    "{} {} {}",
                    dotfile.source().display(),
                    "→".green().bold(),
                    dotfile.target().display()
                );
            }
        }

        let services_header = "Services".blue().bold().to_string();
        let services = config.services().services();
        println!("\n{services_header}");
        if services.is_empty() {
            let msg = "None".yellow().to_string();
            println!("{msg}");
        } else {
            let names: Vec<String> = services.iter().map(Service::service).collect();
            println!("{}", names.join(", "));
        }

        let users_header = "Users".blue().bold().to_string();
        let users = config.users().users();
        println!("\n{users_header}");
        if users.is_empty() {
            let msg = "None".yellow().to_string();
            println!("{msg}");
        } else {
            for user in users {
                let user_text = user.name().blue().underline().to_string();
                let groups = user.groups();
                if groups.is_empty() {
                    let msg = "No groups".yellow().to_string();
                    println!("{user_text}:\n{msg}\n");
                } else {
                    println!("{}:\n{}\n", user_text, groups.join(", "));
                }
            }
        }
    }
}
