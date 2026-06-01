use crate::domain::{Item, Mutation, Mutation::Retain, Page, Pageable};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct ItemInventory {
    pub id: Uuid,
    pub item: Item,
    pub quantity: i32,
}

impl ItemInventory {
    pub fn of(
        id: Uuid,
        item: Item,
        quantity: i32
    ) -> Self {
        Self {
            id,
            item,
            quantity,
        }
    }

    pub fn new(
        item: Item,
        quantity: i32
    ) -> Self {
        Self::of(
            Uuid::now_v7(),
            item,
            quantity,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ItemInventoryMutation {
    pub id: Uuid,
    pub item: Mutation<Item>,
    pub quantity: Mutation<i32>,
}

impl ItemInventoryMutation {
    pub fn of(
        id: Uuid,
        item: Mutation<Item>,
        quantity: Mutation<i32>,
    ) -> Self {
        Self {
            id,
            item,
            quantity,
        }
    }

    pub fn is_all_retained(&self) -> bool {
        matches!(self.item, Retain) && matches!(self.quantity, Retain)
    }
}

pub enum ItemInventoryCommand {
    Create(ItemInventory),
    Replace(ItemInventory),
    Modify(ItemInventoryMutation),
}

#[async_trait]
pub trait ItemInventoryRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<ItemInventory>>;
    async fn find_by_item_id(&self, item_id: &Uuid) -> anyhow::Result<Option<ItemInventory>>;
    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>>;
    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>>;
    // Only when supporting partial updates via HTTP PATCH should Command be used.
    async fn save(&self, item_inventory: &ItemInventoryCommand) -> anyhow::Result<ItemInventory>;
}
