use PEQUOD::simulation::Simulation;
use PEQUOD::config::{SimulationConfig, ConfigError};

fn main() -> Result<(), AppError> {

    let config = SimulationConfig::from_toml("config/pequod.toml")?;
    let simulation = Simulation::new(config); // Will need to make this mutable in future

    simulation.run();

    Ok(())
}

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError)
}

impl From<ConfigError> for AppError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}