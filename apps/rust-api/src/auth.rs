use async_trait::async_trait;
use axum::extract::FromRef;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: Option<String>,
    pub aud: Option<serde_json::Value>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OidcConfig {
    pub issuer: String,
    pub client_id: String,
    pub jwks_url: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Jwk {
    kid: String,
    kty: String,
    alg: Option<String>,
    n: Option<String>,
    e: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Clone)]
pub struct AuthState {
    http: Client,
    config: OidcConfig,
    jwks_cache: Arc<RwLock<HashMap<String, DecodingKey>>>,
}

impl AuthState {
    pub fn new(config: OidcConfig) -> Self {
        Self {
            http: Client::new(),
            config,
            jwks_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn refresh_jwks(&self) -> anyhow::Result<()> {
        let jwks: Jwks = self.http.get(&self.config.jwks_url).send().await?.json().await?;
        let mut cache = self.jwks_cache.write().await;
        cache.clear();
        for k in jwks.keys.into_iter() {
            if let (Some(n), Some(e)) = (k.n.as_deref(), k.e.as_deref()) {
                if let Ok(dec_key) = DecodingKey::from_rsa_components(n, e) {
                    cache.insert(k.kid, dec_key);
                }
            }
        }
        Ok(())
    }

    async fn decoding_key_for(&self, kid: &str) -> anyhow::Result<Option<DecodingKey>> {
        if let Some(k) = self.jwks_cache.read().await.get(kid).cloned() {
            return Ok(Some(k));
        }
        self.refresh_jwks().await?;
        Ok(self.jwks_cache.read().await.get(kid).cloned())
    }

    pub async fn validate_bearer(&self, bearer_token: &str) -> anyhow::Result<Claims> {
        // Extract header to get kid
        let header = jsonwebtoken::decode_header(bearer_token)?;
        let kid = header.kid.ok_or_else(|| anyhow::anyhow!("missing kid"))?;
        let key = self
            .decoding_key_for(&kid)
            .await?
            .ok_or_else(|| anyhow::anyhow!("no key for kid"))?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[self.config.issuer.clone()]);
        // Disable strict audience validation to accommodate Keycloak tokens
        validation.set_audience::<&str>(&[]);
        let data = decode::<Claims>(bearer_token, &key, &validation)?;
        Ok(data.claims)
    }
}

pub struct AuthUser(pub Claims);

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<AuthState>: axum::extract::FromRef<S>,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_state = Arc::<AuthState>::from_ref(state);
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
        let claims = auth_state
            .validate_bearer(token)
            .await
            .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
        Ok(AuthUser(claims))
    }
}
