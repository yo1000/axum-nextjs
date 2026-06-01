use crate::security::authorization::Principal;
use crate::security::AuthError::{Anonymous, NotPermitted};
use anyhow::anyhow;
use std::sync::Arc;
use tokio::task_local;

task_local! {
    static PRINCIPAL: Option<Arc<Principal>>;
}

pub struct SecurityContext;

impl SecurityContext {
    pub async fn scope<F, R>(principal: Option<Principal>, f: F) -> R
    where
        F: Future<Output = R>,
    {
        PRINCIPAL.scope(principal.map(Arc::new), f).await
    }

    pub fn current_principal() -> anyhow::Result<Option<Arc<Principal>>> {
        PRINCIPAL
            .try_with(|principal| principal.clone())
            .map_err(|_| anyhow!("Principal not found in current scope"))
    }

    pub fn check_principal(allow_anonymous: bool, allow_any_user: bool, allow_users: &[&str], allow_roles: &[&str]) -> anyhow::Result<()> {
        if allow_anonymous {
            return Ok(())
        }

        let principal = Self::current_principal()?;

        match principal {
            Some(p) => {
                if allow_any_user {
                    return Ok(())
                }

                if allow_users.contains(&p.username.as_str()) {
                    return Ok(())
                }

                for role in &p.roles {
                    if allow_roles.contains(&role.as_str()) {
                        return Ok(())
                    }
                }

                Err(anyhow!(NotPermitted(
                    format!("username={:?}, roles={:?}", p.username, p.roles))))
            }
            None => {
                Err(anyhow!(Anonymous))
            }
        }
    }
}
