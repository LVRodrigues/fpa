use std::sync::Arc;

use crate::{
    ctx::Context,
    error::Error,
    jwks,
    model::{prelude::Users, users},
    state::AppState,
};

use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use log::{debug, info};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const BEARER: &str = "Bearer ";
const AUDIENCE: &str = "account";

/**
 * Claims is used to extract information from the Token.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    tenant: Uuid,
    name: String,
    email: String,
    iss: String,
}

impl Claims {
    fn to_context(&self) -> Context {
        Context::new(
            self.sub,
            self.tenant,
            self.name.to_owned(),
            self.email.to_owned(),
        )
    }
}

pub async fn require(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    debug!("Validating the user token.");

    if !jwks::is_prepared() {
        return Err(Error::JWKSNotFound);
    }

    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            if value.starts_with(BEARER) {
                Some(value[BEARER.len()..].to_owned())
            } else {
                None
            }
        });
    let token = match token {
        Some(v) => v,
        None => return Err(Error::Unauthorized),
    };

    let header = decode_header(&token)?;

    let kid = header.kid.ok_or(Error::TokenInvalid)?;
    let unverified_issuer = token.split('.').nth(1)
        .and_then(|payload| {
            use base64::Engine;
            base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload).ok()
        })
        .and_then(|payload| serde_json::from_slice::<serde_json::Value>(&payload).ok())
        .and_then(|claims| claims.get("iss")?.as_str().map(str::to_owned))
        .ok_or(Error::TokenInvalid)?;
    if !jwks::issuers(state.configuration()).iter().any(|issuer| issuer == &unverified_issuer) {
        return Err(Error::TokenInvalid);
    }
    let key = jwks::key(&unverified_issuer, &kid)?;
    let key = DecodingKey::from_jwk(&key).map_err(|_| Error::TokenInvalid)?;

    let mut validation = Validation::new(header.alg);
    validation.set_audience(&[AUDIENCE]);
    validation.set_issuer(&[unverified_issuer.as_str()]);
    validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

    let claims = decode::<Claims>(&token, &key, &validation)?.claims;
    request.extensions_mut().insert(claims.to_context());

    Ok(next.run(request).await)
}

pub async fn user_register(
    context: Option<Context>,
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    debug!("Registering the new user.");

    let ctx = context.ok_or(Error::ContextInvalid)?;

    let db = state.connection(ctx.tenant()).await?;
    let user = match Users::find_by_id(*ctx.id()).one(&db).await {
        Ok(u) => u,
        Err(_) => return Err(Error::DatabaseConnection),
    };
    if user.is_none() {
        let u = users::ActiveModel {
            user: Set(ctx.id().clone()),
            name: Set(ctx.name().to_string()),
            tenant: Set(ctx.tenant().clone()),
            time: Set(Utc::now().into()),
            email: Set(ctx.email().to_string()),
        };
        let _ = match u.insert(&db).await {
            Ok(v) => info!("New User: {:?}", v),
            Err(_) => return Err(Error::RegisterUser),
        };
        match db.commit().await {
            Ok(it) => it,
            Err(_) => return Err(Error::DatabaseTransaction),
        };
    }

    Ok(next.run(request).await)
}
