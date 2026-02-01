use pyo3::FromPyObject;

#[derive(FromPyObject, Debug, Clone)]
pub struct Config {
    system: System,
    packages: Packages,
}

impl Config {
    pub fn get_system_config(&self) -> System {
        self.system.clone()
    }
}

#[derive(FromPyObject, Debug, Clone)]
pub struct System {
    hostname: String,
}

impl System {
    pub fn get_configured_hostname(&self) -> String {
        self.hostname.clone()
    }
}

#[derive(FromPyObject, Debug, Clone, Default)]
pub struct Packages {
    pacman: Vec<String>,
    aur: Vec<String>,
}

impl Packages {
    pub fn get_configured_pacman_pkgs(&self) -> Vec<String> {
        self.pacman.clone()
    }

    pub fn get_configured_aur_pkgs(&self) -> Vec<String> {
        self.aur.clone()
    }
}
