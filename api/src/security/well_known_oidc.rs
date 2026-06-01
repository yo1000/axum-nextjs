use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WellKnownOidc {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub introspection_endpoint: Option<String>,
    pub userinfo_endpoint: Option<String>,
    pub end_session_endpoint: Option<String>,
    pub registration_endpoint: Option<String>,
    pub revocation_endpoint: Option<String>,
    pub jwks_uri: Option<String>,
}
