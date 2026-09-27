use std::{error::Error, path::Path, str::FromStr};

use axum::http::uri::Scheme;
use config::{Config, File};
use log::info;

#[derive(Debug, Clone)]
pub struct ConfigurationDatabase {
    pub engine: String,
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
    pub connections_max: u32,
    pub connections_min: u32,
    pub timeout_connect: u64,
    pub timeout_acquire: u64,
    pub timeout_idle: u64,
    pub lifetime: u64,
}

#[derive(Debug, Clone)]
pub struct Empiricals {
    pub productivity: i32,
    pub coordination: i32,
    pub deployment: i32,
    pub planning: i32,
    pub testing: i32,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub scheme: Scheme,
    pub authority: String,
    pub port: u16,
    pub jwks: Vec<JwksConfiguration>,
    pub database: ConfigurationDatabase,
    pub empiricals: Empiricals,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JwksConfiguration {
    pub issuer: String,
    pub url: String,
}

pub fn prepare() -> Result<Configuration, Box<dyn Error + Send + Sync>> {
    info!("Configuring fpa-server...");
    let settings = Config::builder()
        .add_source(File::from(Path::new("config.yaml")))
        .build()?;

    let scheme: String = settings.get("scheme")?;
    let configuration = Configuration {
        scheme: Scheme::from_str(scheme.as_str())?,
        authority: settings.get("authority")?,
        port: settings.get("port")?,
        jwks: settings.get("jwks")?,
        database: ConfigurationDatabase {
            engine: settings.get("database.engine")?,
            server: settings.get("database.server")?,
            port: settings.get("database.port")?,
            username: settings.get("database.username")?,
            password: settings.get("database.password")?,
            name: settings.get("database.name")?,
            connections_max: settings.get("database.connections_max")?,
            connections_min: settings.get("database.connections_min")?,
            timeout_connect: settings.get("database.timeout_connect")?,
            timeout_acquire: settings.get("database.timeout_acquire")?,
            timeout_idle: settings.get("database.timeout_idle")?,
            lifetime: settings.get("database.lifetime")?,
        },
        empiricals: Empiricals {
            productivity: settings.get("empiricals.productivity")?,
            coordination: settings.get("empiricals.coordination")?,
            deployment: settings.get("empiricals.deployment")?,
            planning: settings.get("empiricals.planning")?,
            testing: settings.get("empiricals.testing")?,
        },
    };
    if configuration.port == 0 || configuration.database.connections_min > configuration.database.connections_max {
        return Err("invalid server or database pool configuration".into());
    }
    if configuration.jwks.is_empty() || configuration.jwks.iter().any(|j| j.issuer.is_empty() || j.url.is_empty()) {
        return Err("at least one valid JWKS issuer and URL must be configured".into());
    }
    Ok(configuration)
}
