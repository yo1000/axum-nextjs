use crate::application_context::Db;
use crate::domain::{Page, Pageable, User, UserCommand, UserMutation, UserRepository};
use crate::security::AuthError::NotPermitted;
use crate::security::SecurityContext;
use crate::transaction::TransactionContext;
use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait UserApplicationService: Send + Sync {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<User>>;
    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<User>>;
    async fn search(&self, username: &String, pageable: &Pageable) -> anyhow::Result<Page<User>>;
    async fn create(&self, user: &User) -> anyhow::Result<User>;
    async fn update(&self, user: &User) -> anyhow::Result<User>;
    async fn update_diff(&self, user_mutation: &UserMutation) -> anyhow::Result<User>;
}

pub struct UserApplicationServiceImpl {
    user_repos: Arc<dyn UserRepository>,
}

impl UserApplicationServiceImpl {
    pub fn new(
        user_repos: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            user_repos,
        }
    }
}

#[async_trait]
impl UserApplicationService for UserApplicationServiceImpl {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<User>> {
        self.user_repos.find_by_id(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        self.user_repos.find_all(pageable).await
    }

    async fn search(&self, username: &String, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        self.user_repos.find_all_by_username_starting_with(username, pageable).await
    }

    async fn create(&self, user: &User) -> anyhow::Result<User> {
        self.user_repos
            .save(&UserCommand::Create(user.clone()))
            .await
    }

    async fn update(&self, user: &User) -> anyhow::Result<User> {
        self.user_repos
            .save(&UserCommand::Replace(user.clone()))
            .await
    }

    async fn update_diff(&self, user_mutation: &UserMutation) -> anyhow::Result<User> {
        self.user_repos
            .save(&UserCommand::Modify(user_mutation.clone()))
            .await
    }
}

pub struct UserApplicationServiceAuthDecorator {
    app_service: Arc<dyn UserApplicationService>,
}

impl UserApplicationServiceAuthDecorator {
    pub fn new(app_service: Arc<dyn UserApplicationService>) -> Self {
        Self { app_service }
    }
}

#[async_trait]
impl UserApplicationService for UserApplicationServiceAuthDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<User>> {
        // Authenticated access is permitted
        SecurityContext::check_principal(
            false,
            true,
            &[],
            &[])?;

        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        // Authenticated access is permitted
        SecurityContext::check_principal(
            false,
            true,
            &[],
            &[])?;

        self.app_service.list(pageable).await
    }

    async fn search(&self, username: &String, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        // Authenticated access is permitted
        SecurityContext::check_principal(
            false,
            true,
            &[],
            &[])?;

        self.app_service.search(username, pageable).await
    }

    async fn create(&self, user: &User) -> anyhow::Result<User> {
        // One of following roles is required
        // - `admin`
        // - `axum-nextjs.user:write`
        SecurityContext::check_principal(
            false,
            false,
            &[],
            &["admin", "axum-nextjs.user:write"])?;

        self.app_service.create(user).await
    }

    async fn update(&self, user: &User) -> anyhow::Result<User> {
        // One of following roles is required
        // - `admin`
        // Self access is permitted
        let exist_user = self.app_service.lookup(&user.id).await?;

        let exist_username = match exist_user {
            Some(u) => u.username,
            None => return Err(anyhow!(NotPermitted("missing user".to_string()))),
        };

        SecurityContext::check_principal(
            false,
            false,
            &[exist_username.as_str()],
            &["admin"])?;

        self.app_service.update(user).await
    }

    async fn update_diff(&self, user_mutation: &UserMutation) -> anyhow::Result<User> {
        // One of following roles is required
        // - `admin`
        // Self access is permitted
        let exist_user = self.app_service.lookup(&user_mutation.id).await?;

        let exist_username = match exist_user {
            Some(u) => u.username,
            None => return Err(anyhow!(NotPermitted("missing user".to_string()))),
        };

        SecurityContext::check_principal(
            false,
            false,
            &[exist_username.as_str()],
            &["admin"])?;

        self.app_service.update_diff(user_mutation).await
    }
}

pub struct UserApplicationServiceTxDecorator {
    app_service: Arc<dyn UserApplicationService>,
    pool: Pool<Db>,
}

impl UserApplicationServiceTxDecorator {
    pub fn new(
        app_service: Arc<dyn UserApplicationService>,
        pool: Pool<Db>,
    ) -> Self {
        Self { app_service, pool }
    }
}

#[async_trait]
impl UserApplicationService for UserApplicationServiceTxDecorator {
    async fn lookup(&self, id: &Uuid) -> anyhow::Result<Option<User>> {
        self.app_service.lookup(id).await
    }

    async fn list(&self, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        self.app_service.list(pageable).await
    }

    async fn search(&self, username: &String, pageable: &Pageable) -> anyhow::Result<Page<User>> {
        self.app_service.search(username, pageable).await
    }

    async fn create(&self, user: &User) -> anyhow::Result<User> {
        let tx = self.pool.begin().await?;

        let (result, tx) = TransactionContext::scope(
            tx, async {
                self.app_service.create(user).await
            })
            .await?;

        match &result {
            Ok(_) => tx.commit().await?,
            Err(e) => tx.rollback().await?,
        };

        result
    }

    async fn update(&self, user: &User) -> anyhow::Result<User> {
        let tx = self.pool.begin().await?;

        let (result, tx) = TransactionContext::scope(
            tx, async {
                self.app_service.update(user).await
            })
            .await?;

        match &result {
            Ok(_) => tx.commit().await?,
            Err(e) => tx.rollback().await?,
        };

        result
    }

    async fn update_diff(&self, user_mutation: &UserMutation) -> anyhow::Result<User> {
        let tx = self.pool.begin().await?;

        let (result, tx) = TransactionContext::scope(
            tx, async {
                self.app_service.update_diff(user_mutation).await
            })
            .await?;

        match &result {
            Ok(_) => tx.commit().await?,
            Err(e) => tx.rollback().await?,
        };

        result
    }
}
