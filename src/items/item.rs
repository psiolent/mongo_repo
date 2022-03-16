use serde::{Deserialize, Serialize};

use crate::common::{Entity, Id};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "_id")]
    id: Id,
    name: String,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ItemSpec {
    pub name: String,
}

impl Entity for Item {
    fn id(&self) -> &Id {
        &self.id
    }
}
