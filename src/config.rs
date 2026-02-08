use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use rustyscript::{Module, Runtime, RuntimeOptions};
use serde::{Deserialize, Serialize};

pub use crate::config::demiurge_config::DemiurgeConfig;
pub use crate::config::dotfile::{Dotfile, Dotfiles};
pub use crate::config::packages::Packages;
pub use crate::config::system::System;

mod demiurge_config;
mod dotfile;
mod packages;
mod system;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Demiurge(HashMap<String, DemiurgeConfig>);

impl Demiurge {
    /// # Errors
    /// TODO
    pub fn from_file(file: PathBuf) -> Result<Self> {
        let module = Module::load(file)?;
        Runtime::execute_module(&module, vec![], RuntimeOptions::default(), &())
            .context("Failed to execute config module")
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<DemiurgeConfig> {
        self.0.get(name).cloned()
    }
}
