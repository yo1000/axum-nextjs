use crate::application_context::Db;
use crate::domain::{Item, ItemRepository, Page, Pageable};
use crate::security::SecurityContext;
use async_trait::async_trait;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait ItemApplicationService: Send + Sync {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Item>>;
    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Item>>;
    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Item>>;
}

pub struct ItemApplicationServiceImpl {
    item_repos: Arc<dyn ItemRepository>,
}

impl ItemApplicationServiceImpl {
    pub fn new(item_repos: Arc<dyn ItemRepository>) -> Self {
        Self { item_repos }
    }
}

#[async_trait]
impl ItemApplicationService for ItemApplicationServiceImpl {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Item>> {
        self.item_repos.find_by_id(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        self.item_repos.find_all(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        self.item_repos.find_all_by_name_starting_with(name, pageable).await
    }
}

pub struct ItemApplicationServiceTxDecorator {
    app_service: Arc<dyn ItemApplicationService>,
    pool: Pool<Db>,
}

impl ItemApplicationServiceTxDecorator {
    pub fn new(
        app_service: Arc<dyn ItemApplicationService>,
        pool: Pool<Db>,
    ) -> Self {
        Self { app_service, pool }
    }
}

#[async_trait]
impl ItemApplicationService for ItemApplicationServiceTxDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Item>> {
        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        self.app_service.search(name, pageable).await
    }
}

pub struct ItemApplicationServiceAuthDecorator {
    app_service: Arc<dyn ItemApplicationService>,
}

impl ItemApplicationServiceAuthDecorator {
    pub fn new(app_service: Arc<dyn ItemApplicationService>) -> Self {
        Self { app_service }
    }
}

#[async_trait]
impl ItemApplicationService for ItemApplicationServiceAuthDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<Item>> {
        // Any access is permitted
        SecurityContext::check_principal(
            true,
            false,
            &[],
            &[])?;

        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        // Any access is permitted
        SecurityContext::check_principal(
            true,
            false,
            &[],
            &[])?;

        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        // Any access is permitted
        SecurityContext::check_principal(
            true,
            false,
            &[],
            &[])?;

        self.app_service.search(name, pageable).await
    }
}
