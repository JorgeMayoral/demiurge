use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use rustyscript::{Module, Runtime, RuntimeOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::config::demiurge_config::DemiurgeConfig;
pub use crate::config::dotfile::{Dotfile, Dotfiles};
pub use crate::config::packages::Packages;
pub use crate::config::service::{Service, Services};
pub use crate::config::system::System;
pub use crate::config::user::{User, Users};

mod demiurge_config;
mod dotfile;
mod packages;
mod service;
mod system;
mod user;

pub(super) const CURRENT_DOTFILES_CONFIG_FILE_NAME: &str = "current_dotfiles_config";
pub(super) const CURRENT_PACKAGES_CONFIG_FILE_NAME: &str = "current_packages_config";
pub(super) const CURRENT_SERVICES_CONFIG_FILE_NAME: &str = "current_services_config";
pub(super) const CURRENT_SYSTEM_CONFIG_FILE_NAME: &str = "current_system_config";
pub(super) const CURRENT_USERS_CONFIG_FILE_NAME: &str = "current_users_config";

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Demiurge(HashMap<String, DemiurgeConfig>);

impl Demiurge {
    pub fn from_file(file: PathBuf) -> Result<Self> {
        let module = Module::load(file).context("load config module")?;
        Runtime::execute_module(&module, vec![], RuntimeOptions::default(), &())
            .context("Failed to execute config module")
    }

    pub fn get(&self, name: &str) -> Option<DemiurgeConfig> {
        self.0.get(name).cloned()
    }
}
