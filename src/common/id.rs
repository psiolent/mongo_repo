use std::fmt::Display;

use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(ObjectId);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ObjectId> for Id {
    fn from(oid: ObjectId) -> Self {
        Id(oid)
    }
}

impl From<Id> for Bson {
    fn from(id: Id) -> Self {
        Bson::ObjectId(id.0)
    }
}
