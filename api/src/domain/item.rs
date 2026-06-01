use crate::domain::pageable::Page;
use crate::domain::Pageable;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub code: i32,
    pub name: String,
    pub price: Option<i32>,
    pub sell_price: Option<i32>,
}

impl Item {
    pub fn of(
        id: Uuid,
        code: i32,
        name: impl Into<String>,
        price: Option<i32>,
        sell_price: Option<i32>
    ) -> Self {
        Self {
            id,
            code,
            name: name.into(),
            price,
            sell_price,
        }
    }

    pub fn new(
        code: i32,
        name: impl Into<String>,
        price: Option<i32>,
        sell_price: Option<i32>
    ) -> Self {
        Self::of(
            Uuid::now_v7(),
            code,
            name,
            price,
            sell_price,
        )
    }
}

#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Item>>;
    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<Item>>;
    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<Item>>;
}
