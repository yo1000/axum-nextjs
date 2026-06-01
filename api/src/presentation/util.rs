use crate::security::AuthError;
use crate::security::AuthError::{Anonymous, NotPermitted};
use anyhow::Error;
use axum::http::StatusCode;

pub fn error_to_status(e: Error) -> StatusCode {
    if e.is::<AuthError>() {
        if let Some(auth_error) = e.downcast_ref::<AuthError>() {
            return match auth_error {
                Anonymous => StatusCode::UNAUTHORIZED,
                NotPermitted(_) => StatusCode::FORBIDDEN,
            }
        }
    }
    StatusCode::INTERNAL_SERVER_ERROR
}
