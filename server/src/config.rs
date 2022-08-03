use config::{ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

const DEFAULTS: &str = include_str!("defaults.toml");

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub listen_address: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(File::from_str(DEFAULTS, FileFormat::Toml))
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
