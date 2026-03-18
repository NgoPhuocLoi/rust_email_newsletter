use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub db: DatabaseSettings,
    pub server_port: u16
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder().add_source(File::with_name("configuration")).build()?;
    
    
    settings.try_deserialize()
}