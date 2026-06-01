use crate::application::item_application_service::{ItemApplicationServiceAuthDecorator, ItemApplicationServiceImpl, ItemApplicationServiceTxDecorator};
use crate::application::item_inventory_application_service::{ItemInventoryApplicationServiceAuthDecorator, ItemInventoryApplicationServiceImpl, ItemInventoryApplicationServiceTxDecorator};
use crate::application::user_application_service::{UserApplicationServiceAuthDecorator, UserApplicationServiceImpl, UserApplicationServiceTxDecorator};
use crate::application::weapon_application_service::{WeaponApplicationServiceAuthDecorator, WeaponApplicationServiceImpl, WeaponApplicationServiceTxDecorator};
use crate::application::weapon_remodel_application_service::{WeaponRemodelApplicationServiceAuthDecorator, WeaponRemodelApplicationServiceImpl, WeaponRemodelApplicationServiceTxDecorator};
use crate::application::{ItemApplicationService, ItemInventoryApplicationService, UserApplicationService, WeaponApplicationService, WeaponRemodelApplicationService};
use crate::config_props::ApplicationProperties;
use crate::infrastructure::{PgItemInventoryRepository, PgItemRepository, PgUserRepository, PgWeaponRemodelRepository, PgWeaponRepository};
use crate::security::WellKnownOidc;
use anyhow::anyhow;
use jsonwebtoken::jwk::JwkSet;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;

pub type Db = Postgres;

#[derive(Clone)]
pub struct ApplicationContext {
    pub props: Arc<ApplicationProperties>,
    pub item_app: Arc<dyn ItemApplicationService>,
    pub item_inventory_app: Arc<dyn ItemInventoryApplicationService>,
    pub weapon_app: Arc<dyn WeaponApplicationService>,
    pub weapon_remodel_app: Arc<dyn WeaponRemodelApplicationService>,
    pub user_app: Arc<dyn UserApplicationService>,
    pub jwk_set: Option<Arc<JwkSet>>,
}

impl ApplicationContext {
    pub async fn new(
        props: Arc<ApplicationProperties>,
    ) -> anyhow::Result<Self> {
        let pool = PgPool::connect(&props.datasource.url).await?;

        let item_repos = Arc::new(PgItemRepository::new(pool.clone()));
        let mut item_app: Arc<dyn ItemApplicationService> = Arc::new(ItemApplicationServiceImpl::new(item_repos));
        item_app = Arc::new(ItemApplicationServiceTxDecorator::new(item_app, pool.clone()));

        let item_inventory_repos = Arc::new(PgItemInventoryRepository::new(pool.clone()));
        let mut item_inventory_app: Arc<dyn ItemInventoryApplicationService> = Arc::new(ItemInventoryApplicationServiceImpl::new(item_inventory_repos));
        item_inventory_app = Arc::new(ItemInventoryApplicationServiceTxDecorator::new(item_inventory_app, pool.clone()));

        let weapon_repos = Arc::new(PgWeaponRepository::new(pool.clone()));
        let mut weapon_app: Arc<dyn WeaponApplicationService> = Arc::new(WeaponApplicationServiceImpl::new(weapon_repos));
        weapon_app = Arc::new(WeaponApplicationServiceTxDecorator::new(weapon_app, pool.clone()));

        let weapon_remodel_repos = Arc::new(PgWeaponRemodelRepository::new(pool.clone()));
        let mut weapon_remodel_app: Arc<dyn WeaponRemodelApplicationService> = Arc::new(WeaponRemodelApplicationServiceImpl::new(weapon_remodel_repos));
        weapon_remodel_app = Arc::new(WeaponRemodelApplicationServiceTxDecorator::new(weapon_remodel_app, pool.clone()));

        let user_repos = Arc::new(PgUserRepository::new(pool.clone()));
        let mut user_app: Arc<dyn UserApplicationService> = Arc::new(UserApplicationServiceImpl::new(user_repos));
        user_app = Arc::new(UserApplicationServiceTxDecorator::new(user_app, pool.clone()));

        if props.security.enabled {
            item_app = Arc::new(ItemApplicationServiceAuthDecorator::new(item_app));
            item_inventory_app = Arc::new(ItemInventoryApplicationServiceAuthDecorator::new(item_inventory_app));
            weapon_app = Arc::new(WeaponApplicationServiceAuthDecorator::new(weapon_app));
            weapon_remodel_app = Arc::new(WeaponRemodelApplicationServiceAuthDecorator::new(weapon_remodel_app));
            user_app = Arc::new(UserApplicationServiceAuthDecorator::new(user_app));
        }

        let jwk_set = match &props.security.jwt {
            Some(jwt_props) => {
                match &jwt_props.issuer_uri {
                    Some(issuer_uri) => {
                        let jwks_uri = resolve_jwks_uri(&issuer_uri).await?;
                        let jwk_set = fetch_jwk_set(&jwks_uri).await?;

                        Some(Arc::new(jwk_set))
                    }
                    None => {
                        if props.security.enabled {
                            return Err(anyhow!("No Issuer URI given"));
                        }

                        None
                    }
                }
            },
            None => {
                if props.security.enabled {
                    return Err(anyhow!("No JWT props given"));
                }

                None
            }
        };

        Ok(Self {
            props,
            item_app,
            item_inventory_app,
            weapon_app,
            weapon_remodel_app,
            user_app,
            jwk_set,
        })
    }
}

async fn resolve_jwks_uri(issuer_uri: &String) -> anyhow::Result<String> {
    let normalized_issuer_uri = issuer_uri.trim_end_matches('/');
    let discovery_uri = format!("{normalized_issuer_uri}/.well-known/openid-configuration");

    let well_known_oidc = reqwest::get(&discovery_uri)
        .await?
        .json::<WellKnownOidc>()
        .await?;

    match well_known_oidc.jwks_uri {
        Some(jwks_uri) => Ok(jwks_uri),
        None => Err(anyhow!("No JWKS URI given")),
    }
}

async fn fetch_jwk_set(jwks_uri: &String) -> anyhow::Result<JwkSet> {
    let jwks = reqwest::get(jwks_uri)
        .await?
        .json::<JwkSet>()
        .await?;

    Ok(jwks)
}
