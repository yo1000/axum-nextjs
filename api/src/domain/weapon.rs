use crate::domain::{Page, Pageable};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Weapon {
    pub id: Uuid,
    pub name: String,
    pub str: i32,
    pub hit: i32,
}

impl Weapon {
    pub fn of(
        id: Uuid,
        name: impl Into<String>,
        str: i32,
        hit: i32,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            str,
            hit,
        }
    }

    pub fn new(
        name: impl Into<String>,
        str: i32,
        hit: i32,
    ) -> Self {
        Self::of(
            Uuid::now_v7(),
            name,
            str,
            hit
        )
    }
}

#[async_trait]
pub trait WeaponRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Weapon>>;
    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<Weapon>>;
    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Weapon>>;
}
