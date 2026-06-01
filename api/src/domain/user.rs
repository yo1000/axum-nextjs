use crate::domain::{Mutation, Mutation::Retain, Page, Pageable};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub given_name: String,
    pub family_name: String,
    pub age: Option<i32>,
    pub gender: Gender,
}

impl User {
    pub fn of(
        id: Uuid,
        username: impl Into<String>,
        given_name: impl Into<String>,
        family_name: impl Into<String>,
        age: Option<i32>,
        gender: Gender,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            given_name: given_name.into(),
            family_name: family_name.into(),
            age,
            gender,
        }
    }

    pub fn new(
        username: impl Into<String>,
        given_name: impl Into<String>,
        family_name: impl Into<String>,
        age: Option<i32>,
        gender: Gender,
    ) -> Self {
        Self::of(
            Uuid::now_v7(),
            username,
            given_name,
            family_name,
            age,
            gender,
        )
    }
}

#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum Gender {
    Male = 1,
    Female = 2,
}

impl Gender {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i32> for Gender {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> anyhow::Result<Self> {
        match value {
            1 => Ok(Gender::Male),
            2 => Ok(Gender::Female),
            _ => Err(anyhow::anyhow!("Unknown Gender code {}", value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserMutation {
    pub id: Uuid,
    pub username: Mutation<String>,
    pub given_name: Mutation<String>,
    pub family_name: Mutation<String>,
    pub age: Mutation<Option<i32>>,
    pub gender: Mutation<Gender>,
}

impl UserMutation {
    pub fn of(
        id: Uuid,
        username: Mutation<String>,
        given_name: Mutation<String>,
        family_name: Mutation<String>,
        age: Mutation<Option<i32>>,
        gender: Mutation<Gender>,
    ) -> Self {
        Self {
            id,
            username,
            given_name,
            family_name,
            age,
            gender,
        }
    }

    pub fn is_all_retained(&self) -> bool {
        matches!(self.username, Retain)
            && matches!(self.given_name, Retain)
            && matches!(self.family_name, Retain)
            && matches!(self.age, Retain)
            && matches!(self.gender, Retain)
    }
}

pub enum UserCommand {
    Create(User),
    Replace(User),
    Modify(UserMutation),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<User>>;
    async fn find_by_username(&self, username: &String) -> anyhow::Result<Option<User>>;
    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<User>>;
    async fn find_all_by_username_starting_with(&self, username: &String, pageable: &Pageable) -> anyhow::Result<Page<User>>;
    async fn save(&self, item_command: &UserCommand) -> anyhow::Result<User>;
}
