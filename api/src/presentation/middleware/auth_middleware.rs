use crate::application_context::ApplicationContext;
use crate::security::{Principal, SecurityContext};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, Request as HttpRequest, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iss: Option<String>,
    pub azp: Option<String>,
    pub preferred_username: Option<String>,
    pub realm_access: Option<Access>,
    pub resource_access: Option<HashMap<String, Access>>,
}

#[derive(Debug, Clone, Deserialize)]
struct Access {
    roles: Vec<String>,
}

impl Claims {
    pub fn user_id(&self) -> String {
        // SUBject
        self.sub.clone()
    }

    pub fn username(&self) -> Option<String> {
        self.preferred_username.clone()
    }

    pub fn expiration_time(&self) -> usize {
        self.exp
    }

    pub fn issuer(&self) -> Option<String> {
        self.iss.clone()
    }

    pub fn client_id(&self) -> Option<String> {
        // AuthoriZed Party
        self.azp.clone()
    }

    pub fn roles(&self) -> Vec<String> {
        let mut roles = Vec::new();

        match &self.realm_access {
            Some(access) => roles.extend(access.roles.iter().cloned()),
            None => {}
        }

        match &self.resource_access {
            Some(hash_map) => {
                roles.extend(hash_map.iter()
                    .flat_map(|(key, access)| {
                        let client = key.clone();
                        access.roles.iter()
                            .map(move |role| format!("{client}.{role}"))
                    }))
            },
            None => {}
        }

        roles
    }
}

pub async fn auth_middleware(
    State(ctx): State<Arc<ApplicationContext>>,
    headers: HeaderMap,
    request: HttpRequest<Body>,
    next: Next,
) -> Response {
    if !ctx.props.security.enabled {
        return next.run(request).await
    }

    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let jwk_set = match &ctx.jwk_set {
        Some(jwk_set) => jwk_set,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let issuer_uri = match &ctx.props.security.jwt {
        Some(jwt_props) => {
            match &jwt_props.issuer_uri {
                Some(issuer_uri) => issuer_uri,
                None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let principal = match token {
        Some(t) => match parse_claims(t, &jwk_set, &issuer_uri) {
            Ok(claims) => {
                debug!("[AUTH] ✓ sub={}", claims.sub);

                let user_id = claims.user_id().clone();
                let username = match claims.username() {
                    Some(username) => username,
                    None => return StatusCode::UNAUTHORIZED.into_response()
                };
                let client_id = match claims.client_id() {
                    Some(client_id) => client_id,
                    None => return StatusCode::UNAUTHORIZED.into_response()
                };

                let principal = Principal {
                    user_id,
                    username,
                    client_id,
                    roles: claims.roles(),
                };

                Some(principal)
            }
            Err(e) => {
                warn!("[AUTH] ✗ Invalid token: {e}");
                return StatusCode::UNAUTHORIZED.into_response()
            }
        }
        None => {
            debug!("[AUTH] No token provided");
            None
        }
    };

    SecurityContext::scope(principal, async move {
        next.run(request).await
    }).await
}

fn parse_claims(token: &str, jwk_set: &JwkSet, issuer_uri: &String) -> anyhow::Result<Claims> {
    let header = decode_header(token)
        .map_err(|e| anyhow::anyhow!("Invalid JWT header: {e}"))?;

    let kid = header.kid
        .ok_or_else(|| anyhow::anyhow!("JWT header missing kid"))?;

    let jwk = jwk_set.find(&kid)
        .ok_or_else(|| anyhow::anyhow!("JWK not found for kid: {kid}"))?;

    let decoding_key = match &jwk.algorithm {
        AlgorithmParameters::RSA(rsa) => {
            DecodingKey::from_jwk(jwk)
                .map_err(|e| anyhow::anyhow!("Failed to create DecodingKey: {e}"))
        }
        _ => Err(anyhow::anyhow!("Unsupported algorithm")),
    }?;

    let algorithm = match header.alg {
        Algorithm::RS256 => Algorithm::RS256,
        Algorithm::RS384 => Algorithm::RS384,
        Algorithm::RS512 => Algorithm::RS512,
        Algorithm::ES256 => Algorithm::ES256,
        Algorithm::ES384 => Algorithm::ES384,
        _ => return Err(anyhow::anyhow!(
            "Unsupported algorithm: {:?}", header.alg
        )),
    };

    let mut validation = Validation::new(algorithm);
    validation.set_issuer(&[&issuer_uri]);
    validation.validate_aud = false;

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature =>
                anyhow::anyhow!("Token has expired"),
            jsonwebtoken::errors::ErrorKind::InvalidSignature =>
                anyhow::anyhow!("Invalid token signature"),
            jsonwebtoken::errors::ErrorKind::InvalidToken =>
                anyhow::anyhow!("Malformed token"),
            _ =>
                anyhow::anyhow!("JWT validation failed: {e}"),
        })?;

    Ok(token_data.claims)
}
