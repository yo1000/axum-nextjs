use crate::application_context::Db;
use crate::domain::{Page, Pageable, Weapon, WeaponRepository};
use crate::security::SecurityContext;
use async_trait::async_trait;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait WeaponApplicationService: Send + Sync {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Weapon>>;
    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Weapon>>;
    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Weapon>>;
}

pub struct WeaponApplicationServiceImpl {
    weapon_repos: Arc<dyn WeaponRepository>,
}

impl WeaponApplicationServiceImpl {
    pub fn new(
        weapon_repos: Arc<dyn WeaponRepository>,
    ) -> Self {
        Self {
            weapon_repos,
        }
    }
}

#[async_trait]
impl WeaponApplicationService for WeaponApplicationServiceImpl {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Weapon>> {
        self.weapon_repos.find_by_id(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        self.weapon_repos.find_all(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        self.weapon_repos.find_all_by_name_starting_with(name, pageable).await
    }
}

pub struct WeaponApplicationServiceTxDecorator {
    app_service: Arc<dyn WeaponApplicationService>,
    pool: Pool<Db>,
}

impl WeaponApplicationServiceTxDecorator {
    pub fn new(
        app_service: Arc<dyn WeaponApplicationService>,
        pool: Pool<Db>,
    ) -> Self {
        Self { app_service, pool }
    }
}

#[async_trait]
impl WeaponApplicationService for WeaponApplicationServiceTxDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Weapon>> {
        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        self.app_service.search(name, pageable).await
    }
}

pub struct WeaponApplicationServiceAuthDecorator {
    app_service: Arc<dyn WeaponApplicationService>,
}

impl WeaponApplicationServiceAuthDecorator {
    pub fn new(app_service: Arc<dyn WeaponApplicationService>) -> Self {
        Self { app_service }
    }
}

#[async_trait]
impl WeaponApplicationService for WeaponApplicationServiceAuthDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Weapon>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.weapon:write`
        // - `axum-nextjs.weapon:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.weapon:write", "axum-nextjs.weapon:read"])?;

        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.weapon:write`
        // - `axum-nextjs.weapon:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.weapon:write", "axum-nextjs.weapon:read"])?;

        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.weapon:write`
        // - `axum-nextjs.weapon:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.weapon:write", "axum-nextjs.weapon:read"])?;

        self.app_service.search(name, pageable).await
    }
}
