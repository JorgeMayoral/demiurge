use pyo3::FromPyObject;

#[derive(FromPyObject, Debug, Clone)]
pub struct Config {
    system: System,
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
