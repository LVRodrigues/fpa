use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
    time::Duration,
};

use jsonwebtoken::jwk::Jwk;
use log::{debug, info, warn};
use serde::Deserialize;
use tokio::{
    sync::{Mutex, RwLock},
    time::Instant,
};

use crate::{
    configuration::{Configuration, JwksConfiguration},
    error::Error,
};

const REFRESH_INTERVAL: Duration = Duration::from_secs(300);
const RETRY_INTERVAL: Duration = Duration::from_secs(30);

#[derive(Debug, Deserialize)]
struct Keys {
    #[serde(default, rename = "keys")]
    items: Vec<Jwk>,
}

struct IssuerKeys {
    client: reqwest::Client,
    url: String,
    keys: RwLock<HashMap<String, Jwk>>,
    // Serializes refreshes and limits requests even when the endpoint fails.
    last_attempt: Mutex<Option<Instant>>,
}

static KEYS: OnceLock<HashMap<String, Arc<IssuerKeys>>> = OnceLock::new();

async fn request_jwks(client: &reqwest::Client, url: &str) -> Result<HashMap<String, Jwk>, Error> {
    let response = client.get(url).send().await?.error_for_status()?;
    let jwks: Keys = response.json().await?;
    let mut keys = HashMap::new();
    for jwk in jwks.items {
        let kid = jwk.common.key_id.clone().ok_or(Error::JWKSNotFound)?;
        if keys.insert(kid, jwk).is_some() {
            return Err(Error::JWKSNotFound);
        }
    }
    if keys.is_empty() {
        return Err(Error::JWKSNotFound);
    }
    Ok(keys)
}

impl IssuerKeys {
    async fn load(client: reqwest::Client, url: String) -> Result<Self, Error> {
        let keys = request_jwks(&client, &url).await?;
        Ok(Self {
            client,
            url,
            keys: RwLock::new(keys),
            last_attempt: Mutex::new(None),
        })
    }

    async fn refresh(&self) -> Result<(), Error> {
        let mut last_attempt = self.last_attempt.lock().await;
        if last_attempt.is_some_and(|last| last.elapsed() < RETRY_INTERVAL) {
            return Ok(());
        }
        *last_attempt = Some(Instant::now());
        // Replace only a complete, valid response; retain cached keys on failure.
        let keys = request_jwks(&self.client, &self.url).await?;
        *self.keys.write().await = keys;
        Ok(())
    }

    async fn key(&self, kid: &str) -> Result<Jwk, Error> {
        if let Some(key) = self.keys.read().await.get(kid).cloned() {
            return Ok(key);
        }
        self.refresh().await?;
        self.keys
            .read()
            .await
            .get(kid)
            .cloned()
            .ok_or(Error::KeyNotFound)
    }
}

pub async fn prepare(config: &Configuration) -> Result<(), Error> {
    info!("Preparing JWKS...");
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|_| Error::JWKSNotFound)?;
    let mut issuers = HashMap::new();

    for source in &config.jwks {
        debug!(
            "Requesting JWKS for issuer {} from {}",
            source.issuer, source.url
        );
        let keys = IssuerKeys::load(client.clone(), source.url.clone()).await?;
        if issuers
            .insert(source.issuer.clone(), Arc::new(keys))
            .is_some()
        {
            return Err(Error::JWKSNotFound);
        }
    }

    KEYS.set(issuers).map_err(|_| Error::JWKSNotFound)?;
    for (issuer, keys) in KEYS.get().ok_or(Error::JWKSNotFound)? {
        let issuer = issuer.clone();
        let keys = Arc::clone(keys);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(REFRESH_INTERVAL).await;
                if let Err(error) = keys.refresh().await {
                    warn!("Failed to refresh JWKS for {issuer}: {error}");
                }
            }
        });
    }
    Ok(())
}

pub async fn key(issuer: &str, kid: &str) -> Result<Jwk, Error> {
    let keys = KEYS.get().ok_or(Error::JWKSNotFound)?;
    keys.get(issuer).ok_or(Error::KeyNotFound)?.key(kid).await
}

pub fn is_prepared() -> bool {
    KEYS.get().is_some()
}

pub fn issuers(config: &Configuration) -> Vec<String> {
    config
        .jwks
        .iter()
        .map(|source: &JwksConfiguration| source.issuer.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
    use serde_json::{json, Value};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone)]
    struct Endpoint {
        response: Arc<RwLock<(StatusCode, Value)>>,
        requests: Arc<AtomicUsize>,
    }

    fn document(kid: &str) -> Value {
        json!({"keys": [{"kty": "RSA", "kid": kid, "n": "AQAB", "e": "AQAB"}]})
    }

    async fn serve(State(endpoint): State<Endpoint>) -> (StatusCode, Json<Value>) {
        endpoint.requests.fetch_add(1, Ordering::SeqCst);
        let (status, value) = endpoint.response.read().await.clone();
        (status, Json(value))
    }

    async fn endpoint() -> (Endpoint, IssuerKeys, tokio::task::JoinHandle<()>) {
        let endpoint = Endpoint {
            response: Arc::new(RwLock::new((StatusCode::OK, document("old")))),
            requests: Arc::new(AtomicUsize::new(0)),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/certs", listener.local_addr().unwrap());
        let router = Router::new()
            .route("/certs", get(serve))
            .with_state(endpoint.clone());
        let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        let keys = IssuerKeys::load(reqwest::Client::new(), url).await.unwrap();
        (endpoint, keys, server)
    }

    #[tokio::test]
    async fn rotation_refreshes_once_for_concurrent_requests_and_throttles_unknown_keys() {
        let (endpoint, keys, server) = endpoint().await;
        *endpoint.response.write().await = (StatusCode::OK, document("new"));
        let (first, second) = tokio::join!(keys.key("new"), keys.key("new"));
        assert_eq!(first.unwrap().common.key_id.as_deref(), Some("new"));
        assert_eq!(second.unwrap().common.key_id.as_deref(), Some("new"));
        assert!(matches!(keys.key("old").await, Err(Error::KeyNotFound)));
        assert!(matches!(keys.key("unknown").await, Err(Error::KeyNotFound)));
        assert_eq!(endpoint.requests.load(Ordering::SeqCst), 2);
        server.abort();
    }

    #[tokio::test]
    async fn failed_or_invalid_refresh_preserves_cache_and_recovers_after_cooldown() {
        let (endpoint, keys, server) = endpoint().await;
        for response in [
            (StatusCode::SERVICE_UNAVAILABLE, json!({})),
            (StatusCode::OK, json!({"keys": []})),
            (
                StatusCode::OK,
                json!({"keys": [document("duplicate")["keys"][0], document("duplicate")["keys"][0]]}),
            ),
        ] {
            *keys.last_attempt.lock().await = None;
            *endpoint.response.write().await = response;
            assert!(keys.key("new").await.is_err());
            let requests = endpoint.requests.load(Ordering::SeqCst);
            assert!(keys.key("old").await.is_ok());
            assert!(keys.key("new").await.is_err());
            assert_eq!(endpoint.requests.load(Ordering::SeqCst), requests);
        }
        *endpoint.response.write().await = (StatusCode::OK, document("recovered"));
        *keys.last_attempt.lock().await = Some(Instant::now() - RETRY_INTERVAL);
        keys.refresh().await.unwrap();
        assert!(keys.key("recovered").await.is_ok());
        assert!(matches!(keys.key("old").await, Err(Error::KeyNotFound)));
        server.abort();
    }
}
