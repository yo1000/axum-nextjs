use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApplicationProperties {
    pub server: ServerProperties,
    pub datasource: DataSourceProperties,
    pub security: SecurityProperties,
}

#[derive(Debug, Deserialize)]
pub struct ServerProperties {
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct DataSourceProperties {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SecurityProperties {
    pub enabled: bool,
    pub jwt: Option<JwtProperties>,
}

#[derive(Debug, Deserialize)]
pub struct JwtProperties {
    pub issuer_uri: Option<String>,
}

pub fn load() -> Result<ApplicationProperties, ConfigError> {
    let profile = std::env::var("APP__PROFILE")
        .unwrap_or_else(|_| "".into());

    Config::builder()
        // .set_default("datasource.url", "db://")?
        // .set_default("security.jwt.issuer_uri", "https://")?
        .set_default("server.port", 3000)?
        .set_default("security.enabled", true)?
        .add_source(File::with_name("application").required(false))
        .add_source(File::with_name(&format!("application-{profile}")).required(false))
        .add_source(Environment::with_prefix("APP")
            .separator("__")
            .try_parsing(true)
            .ignore_empty(true),
        )
        .build()?
        .try_deserialize()
}
