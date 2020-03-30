use std::str::FromStr;
use serde::{Deserialize, Deserializer, de::Error};
use config::{Config, ConfigError, File, Environment};
use log::LevelFilter;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub app_address: String,
    #[serde(deserialize_with = "deserialize_log_level")]
    pub log_level: LevelFilter,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings
            // Add in `./Settings.toml`
            .merge(File::with_name("Settings"))?
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .merge(Environment::with_prefix("APP"))?;
        settings.try_into()
    }
}

fn deserialize_log_level<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>
{
    let level = String::deserialize(deserializer)?;
    LevelFilter::from_str(&level).map_err(Error::custom)
}