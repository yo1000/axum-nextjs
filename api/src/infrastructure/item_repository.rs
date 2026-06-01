use crate::domain::{Item, ItemRepository, Page, Pageable};
use crate::transaction::TransactionContext;
use async_trait::async_trait;
use sqlx::{query_as, query_scalar, PgPool};
use uuid::Uuid;

pub struct PgItemRepository {
    pool: PgPool,
}

impl PgItemRepository {
    pub fn new(
        pool: PgPool
    ) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemRepository for PgItemRepository {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Item>> {
        let item = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, Item>(r#"
                    SELECT
                        id,
                        code,
                        name,
                        price,
                        sell_price
                    FROM
                        items
                    WHERE
                        id = $1
                    "#)
                    .bind(id)
                    .fetch_optional(conn)
                    .await
            })
            .await?;

        Ok(item)
    }

    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        items
                    "#)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let items = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, Item>(r#"
                    SELECT
                        id,
                        code,
                        name,
                        price,
                        sell_price
                    FROM
                        items
                    LIMIT $1
                    OFFSET $2
                    "#)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?;

        Ok(Page::new(items, pageable, count))
    }

    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Item>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        items
                    WHERE
                        name LIKE $1 || '%'
                    "#)
                    .bind(name)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let items = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, Item>(r#"
                    SELECT
                        id,
                        code,
                        name,
                        price,
                        sell_price
                    FROM
                        items
                    WHERE
                        name LIKE $1 || '%'
                    LIMIT $2
                    OFFSET $3
                    "#)
                    .bind(name)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?;

        Ok(Page::new(items, pageable, count))
    }
}
