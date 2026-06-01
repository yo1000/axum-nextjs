use crate::application_context::Db;
use crate::domain::{ItemInventory, ItemInventoryCommand, ItemInventoryMutation, ItemInventoryRepository, Page, Pageable};
use crate::security::SecurityContext;
use crate::transaction::TransactionContext;
use async_trait::async_trait;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait ItemInventoryApplicationService: Send + Sync {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<ItemInventory>>;
    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>>;
    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>>;
    async fn create(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory>;
    async fn update(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory>;
    async fn update_diff(&self, item_inventory_mutation: &ItemInventoryMutation) -> anyhow::Result<ItemInventory>;
}

pub struct ItemInventoryApplicationServiceImpl {
    item_inventory_repos: Arc<dyn ItemInventoryRepository>,
}

impl ItemInventoryApplicationServiceImpl {
    pub fn new(item_inventory_repos: Arc<dyn ItemInventoryRepository>) -> Self {
        Self { item_inventory_repos }
    }
}

#[async_trait]
impl ItemInventoryApplicationService for ItemInventoryApplicationServiceImpl {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<ItemInventory>> {
        self.item_inventory_repos.find_by_id(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        self.item_inventory_repos.find_all(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        self.item_inventory_repos.find_all_by_name_starting_with(name, pageable).await
    }

    async fn create(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory> {
        self.item_inventory_repos
            .save(&ItemInventoryCommand::Create(item_inventory.clone()))
            .await
    }

    async fn update(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory> {
        self.item_inventory_repos
            .save(&ItemInventoryCommand::Replace(item_inventory.clone()))
            .await
    }

    async fn update_diff(&self, item_inventory_mutation: &ItemInventoryMutation) -> anyhow::Result<ItemInventory> {
        self.item_inventory_repos
            .save(&ItemInventoryCommand::Modify(item_inventory_mutation.clone()))
            .await
    }
}

pub struct ItemInventoryApplicationServiceTxDecorator {
    app_service: Arc<dyn ItemInventoryApplicationService>,
    pool: Pool<Db>,
}

impl ItemInventoryApplicationServiceTxDecorator {
    pub fn new(
        app_service: Arc<dyn ItemInventoryApplicationService>,
        pool: Pool<Db>,
    ) -> Self {
        Self { app_service, pool }
    }
}

#[async_trait]
impl ItemInventoryApplicationService for ItemInventoryApplicationServiceTxDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<ItemInventory>> {
        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        self.app_service.search(name, pageable).await
    }

    async fn create(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory> {
        let tx = self.pool.begin().await?;

        let (result, tx) = TransactionContext::scope(
            tx, async {
                self.app_service.create(item_inventory).await
            })
            .await?;

        match &result {
            Ok(_) => tx.commit().await?,
            Err(e) => tx.rollback().await?,
        };

        result
    }

    async fn update(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory> {
        let tx = self.pool.begin().await?;

        let (result, tx) = TransactionContext::scope(
            tx, async {
                self.app_service.update(item_inventory).await
            })
            .await?;

        match &result {
            Ok(_) => tx.commit().await?,
            Err(e) => tx.rollback().await?,
        };

        result
    }

    async fn update_diff(&self, item_inventory_mutation: &ItemInventoryMutation) -> anyhow::Result<ItemInventory> {
        let tx = self.pool.begin().await?;

        let (result, tx) = TransactionContext::scope(
            tx, async {
                self.app_service.update_diff(item_inventory_mutation).await
            })
            .await?;

        match &result {
            Ok(_) => tx.commit().await?,
            Err(e) => tx.rollback().await?,
        };

        result
    }
}

pub struct ItemInventoryApplicationServiceAuthDecorator {
    app_service: Arc<dyn ItemInventoryApplicationService>,
}

impl ItemInventoryApplicationServiceAuthDecorator {
    pub fn new(app_service: Arc<dyn ItemInventoryApplicationService>) -> Self {
        Self { app_service }
    }
}

#[async_trait]
impl ItemInventoryApplicationService for ItemInventoryApplicationServiceAuthDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<ItemInventory>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.item_inventory:write`
        // - `axum-nextjs.item_inventory:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.item_inventory:write", "axum-nextjs.item_inventory:read"])?;

        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.item_inventory:write`
        // - `axum-nextjs.item_inventory:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.item_inventory:write", "axum-nextjs.item_inventory:read"])?;

        self.app_service.list(pageable).await
    }

    async fn search(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.item_inventory:write`
        // - `axum-nextjs.item_inventory:read`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.item_inventory:write", "axum-nextjs.item_inventory:read"])?;

        self.app_service.search(name, pageable).await
    }

    async fn create(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.item_inventory:write`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.item_inventory:write"])?;

        self.app_service.create(item_inventory).await
    }

    async fn update(&self, item_inventory: &ItemInventory) -> anyhow::Result<ItemInventory> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.item_inventory:write`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.item_inventory:write"])?;

        self.app_service.update(item_inventory).await
    }

    async fn update_diff(&self, item_inventory_mutation: &ItemInventoryMutation) -> anyhow::Result<ItemInventory> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.item_inventory:write`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.item_inventory:write"])?;

        self.app_service.update_diff(item_inventory_mutation).await
    }
}
