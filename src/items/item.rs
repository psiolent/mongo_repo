use serde::{Deserialize, Serialize};

use crate::common::{Entity, Id};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Item {
    id: Id,
    name: String,
}

impl Item {
    pub fn new(id: Id, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Entity for Item {
    fn id(&self) -> &Id {
        &self.id
    }
}
