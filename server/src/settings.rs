use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub app_address: String,
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