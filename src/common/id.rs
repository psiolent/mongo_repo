use std::{fmt::Display, str::FromStr};

use mongodb::bson::{
    oid::{self, ObjectId},
    Bson,
};
use serde::{Deserialize, Serialize};

/// A unique identifier for identifying an entity.
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
    type Err = oid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ObjectId::parse_str(s)?.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_ids_are_unique() {
        let id1: Id = ObjectId::new().into();
        let id2: Id = ObjectId::new().into();
        assert_ne!(id1, id2);
    }

    #[test]
    fn id_is_equal_after_serializing_and_deserializing() {
        let orig_id: Id = ObjectId::new().into();
        let ser_id = serde_json::to_string(&orig_id).unwrap();
        let deser_id_result = serde_json::from_str::<Id>(ser_id.as_str());

        assert!(deser_id_result.is_ok());
        let deser_id = deser_id_result.unwrap();

        assert_eq!(orig_id, deser_id);
    }
}
