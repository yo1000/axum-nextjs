use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub enum Mutation<T> {
    Retain,
    Clear,
    Assign(T),
}

impl<T> Default for Mutation<T> {
    fn default() -> Self {
        Mutation::Retain
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mutation<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Ok(match Option::<T>::deserialize(deserializer)? {
            Some(v) => Mutation::Assign(v),
            None => Mutation::Clear,
        })
    }
}
