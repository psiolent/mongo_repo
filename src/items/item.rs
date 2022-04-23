use serde::{Deserialize, Serialize};

use crate::common::{entity::Entity, id::Id, name::Name};

#[derive(Deserialize)]
pub struct Item {
    #[serde(rename = "_id")]
    id: Id,
    name: Name,
    size: ItemSize,
}

#[derive(Serialize, Deserialize)]
pub enum ItemSize {
    Small,
    Medium,
    Large,
}

impl Item {
    pub fn new(id: Id, name: Name, size: ItemSize) -> Self {
        Self { id, name, size }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn size(&self) -> &ItemSize {
        &self.size
    }
}

impl Entity for Item {
    fn id(&self) -> &Id {
        &self.id
    }
}
