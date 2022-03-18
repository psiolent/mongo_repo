use std::{fmt::Display, str::FromStr};

use mongodb::bson::{self, oid::ObjectId, Bson};
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

impl FromStr for Id {
    type Err = bson::oid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ObjectId::parse_str(s)?.into())
    }
}
