use crate::domain::{Page, Pageable, Weapon, WeaponRepository};
use crate::transaction::TransactionContext;
use async_trait::async_trait;
use sqlx::{query_as, query_scalar, PgPool};
use uuid::Uuid;

pub struct PgWeaponRepository {
    pool: PgPool,
}

impl PgWeaponRepository {
    pub fn new(
        pool: PgPool
    ) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WeaponRepository for PgWeaponRepository {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Weapon>> {
        let weapon = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, Weapon>(r#"
                    SELECT
                        id,
                        name,
                        str,
                        hit
                    FROM
                        weapons
                    WHERE
                        id = $1
                    "#)
                    .bind(id)
                    .fetch_optional(conn)
                    .await
            })
            .await?;

        Ok(weapon)
    }

    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        weapons
                    "#)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let weapons = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, Weapon>(r#"
                    SELECT
                        id,
                        name,
                        str,
                        hit
                    FROM
                        weapons
                    LIMIT $1
                    OFFSET $2
                    "#)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?;

        Ok(Page::new(weapons, pageable, count))
    }

    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Weapon>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        weapons
                    WHERE
                        name LIKE $1 || '%'
                    "#)
                    .bind(name)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let weapons = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, Weapon>(r#"
                    SELECT
                        id,
                        name,
                        str,
                        hit
                    FROM
                        weapons
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

        Ok(Page::new(weapons, pageable, count))
    }
}
