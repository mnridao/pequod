use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct SimulationConfig {
    pub domain: DomainConfig,
    pub time: SolverConfig,
}

#[derive(Debug, Deserialize)]
pub struct DomainConfig {
    pub grid: GridConfig,
}

#[derive(Debug, Deserialize)]
pub struct SolverConfig {
    pub dt: f64, 
    pub nt: f64
}

#[derive(Debug, Deserialize)]
pub struct GridConfig {
    nx: usize, 
    ny: usize, 
    basinscale: f64
}

#[derive(Debug)]
pub enum ConfigError {
    IO(std::io::Error),
    TOML(toml::de::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        Self::TOML(error)
    }
}

impl SimulationConfig {
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        // path size not known at compile time, so should be passed as reference.
        let path_ref = path.as_ref();
        let config_str = fs::read_to_string(path_ref)?;
        let config: SimulationConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}