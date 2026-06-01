use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Principal {
    pub user_id: String,
    pub username: String,
    pub client_id: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Anonymous access is not allowed")]
    Anonymous,
    #[error("Not permitted: {0}")]
    NotPermitted(String),
}
