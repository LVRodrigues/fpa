use std::{collections::HashMap, sync::{Arc, OnceLock, RwLock}};

use jsonwebtoken::jwk::Jwk;
use log::{debug, info};
use serde::Deserialize;

use crate::{configuration::{Configuration, JwksConfiguration}, error::Error};

#[derive(Debug, Deserialize)]
struct Keys {
    #[serde(default, rename = "keys")]
    items: Vec<Jwk>,
}

#[derive(Debug, Clone)]
struct IssuerKeys {
    keys: HashMap<String, Jwk>,
}

static KEYS: OnceLock<Arc<RwLock<HashMap<String, IssuerKeys>>>> = OnceLock::new();

async fn request_jwks(client: &reqwest::Client, url: &str) -> Result<Keys, Error> {
    let response = client.get(url).send().await?.error_for_status()?;
    Ok(response.json().await?)
}

pub async fn prepare(config: &Configuration) -> Result<(), Error> {
    info!("Preparing JWKS...");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|_| Error::JWKSNotFound)?;
    let mut issuers = HashMap::new();

    for source in &config.jwks {
        debug!("Requesting JWKS for issuer {} from {}", source.issuer, source.url);
        let jwks = request_jwks(&client, &source.url).await?;
        let mut keys = HashMap::new();
        for jwk in jwks.items {
            let kid = jwk.common.key_id.clone().ok_or(Error::JWKSNotFound)?;
            if keys.insert(kid, jwk).is_some() {
                return Err(Error::JWKSNotFound);
            }
        }
        if keys.is_empty() || issuers.insert(source.issuer.clone(), IssuerKeys { keys }).is_some() {
            return Err(Error::JWKSNotFound);
        }
    }

    KEYS.set(Arc::new(RwLock::new(issuers))).map_err(|_| Error::JWKSNotFound)?;
    Ok(())
}

pub fn key(issuer: &str, kid: &str) -> Result<Jwk, Error> {
    let keys = KEYS.get().ok_or(Error::JWKSNotFound)?;
    let keys = keys.read().map_err(|_| Error::JWKSNotFound)?;
    keys.get(issuer)
        .and_then(|source| source.keys.get(kid))
        .cloned()
        .ok_or(Error::KeyNotFound)
}

pub fn is_prepared() -> bool {
    KEYS.get().is_some()
}

pub fn issuers(config: &Configuration) -> Vec<String> {
    config.jwks.iter().map(|source: &JwksConfiguration| source.issuer.clone()).collect()
}
