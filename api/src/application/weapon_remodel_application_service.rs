use crate::application_context::Db;
use crate::domain::{Page, Pageable, WeaponRemodel, WeaponRemodelRepository};
use crate::security::SecurityContext;
use async_trait::async_trait;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait WeaponRemodelApplicationService: Send + Sync {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<WeaponRemodel>>;
    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>>;
    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>>;
}

pub struct WeaponRemodelApplicationServiceImpl {
    weapon_remodel_repos: Arc<dyn WeaponRemodelRepository>,
}

impl WeaponRemodelApplicationServiceImpl {
    pub fn new(weapon_remodel_repos: Arc<dyn WeaponRemodelRepository>) -> Self {
        Self { weapon_remodel_repos }
    }
}

#[async_trait]
impl WeaponRemodelApplicationService for WeaponRemodelApplicationServiceImpl {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<WeaponRemodel>> {
        self.weapon_remodel_repos.find_by_id(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        self.weapon_remodel_repos.find_all(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        self.weapon_remodel_repos.find_all_by_name_starting_with(name, pageable).await
    }
}

pub struct WeaponRemodelApplicationServiceTxDecorator {
    app_service: Arc<dyn WeaponRemodelApplicationService>,
    pool: Pool<Db>,
}

impl WeaponRemodelApplicationServiceTxDecorator {
    pub fn new(
        app_service: Arc<dyn WeaponRemodelApplicationService>,
        pool: Pool<Db>,
    ) -> Self {
        Self { app_service, pool }
    }
}

#[async_trait]
impl WeaponRemodelApplicationService for WeaponRemodelApplicationServiceTxDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<WeaponRemodel>> {
        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        self.app_service.search(name, pageable).await
    }
}

pub struct WeaponRemodelApplicationServiceAuthDecorator {
    app_service: Arc<dyn WeaponRemodelApplicationService>,
}

impl WeaponRemodelApplicationServiceAuthDecorator {
    pub fn new(app_service: Arc<dyn WeaponRemodelApplicationService>) -> Self {
        Self { app_service }
    }
}

#[async_trait]
impl WeaponRemodelApplicationService for WeaponRemodelApplicationServiceAuthDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<WeaponRemodel>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.weapon_remodel:write`
        // - `axum-nextjs.weapon_remodel:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.weapon_remodel:write", "axum-nextjs.weapon_remodel:read"])?;

        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.weapon_remodel:write`
        // - `axum-nextjs.weapon_remodel:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.weapon_remodel:write", "axum-nextjs.weapon_remodel:read"])?;

        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.weapon_remodel:write`
        // - `axum-nextjs.weapon_remodel:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.weapon_remodel:write", "axum-nextjs.weapon_remodel:read"])?;

        self.app_service.search(name, pageable).await
    }
}
