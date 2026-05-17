use std::time::Duration;

use config::{Config, ConfigError, File};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::domain::SubscriberEmail;

#[derive(Deserialize)]
pub struct Settings {
    pub db: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailClientSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub server_port: u16,
    pub server_host: String,
}

#[derive(Deserialize)]
pub struct EmailClientSettings {
    pub sender_email: String,
    pub base_url: String,
    pub api_key: SecretString,
    pub timeout_miliseconds: u64,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_miliseconds)
    }
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name
        ))
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?;

    settings.try_deserialize()
}
