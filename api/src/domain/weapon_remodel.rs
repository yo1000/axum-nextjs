use crate::domain::{Item, Page, Pageable, Weapon};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeaponRemodel {
    pub id: Uuid,
    pub weapon: Weapon,
    pub price: i32,
    pub materials: Vec<WeaponMaterial>,
}

impl WeaponRemodel {
    pub fn of(
        id: Uuid,
        weapon: Weapon,
        price: i32,
        materials: Vec<WeaponMaterial>,
    ) -> Self {
        Self {
            id,
            weapon,
            price,
            materials,
        }
    }

    pub fn new(
        weapon: Weapon,
        price: i32,
        materials: Vec<WeaponMaterial>,
    ) -> Self {
        Self::of(
            Uuid::now_v7(),
            weapon,
            price,
            materials,
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeaponMaterial {
    pub id: Uuid,
    pub item: Item,
    pub quantity: i32,
}

impl WeaponMaterial {
    pub fn of(
        id: Uuid,
        item: Item,
        quantity: i32,
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

#[async_trait]
pub trait WeaponRemodelRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<WeaponRemodel>>;
    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>>;
    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>>;
}
