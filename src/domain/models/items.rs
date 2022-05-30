pub use filter::*;
pub use patch::*;
pub use spec::*;

use serde::{Deserialize, Serialize};

use crate::common::{entity::Entity, id::Id, name::Name};

const MONGO_DB: &str = "repotest";
const MONGO_COLLECTION: &str = "items";

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

mod repo {
    use super::*;
    use crate::storage::{mongo_repo::MongoReposable, repo::Reposable};

    impl Reposable for Item {
        type Spec = ItemSpec;
        type Patch = ItemPatch;
        type Filter = ItemFilter;
    }

    impl MongoReposable for Item {
        fn db_name() -> &'static str {
            MONGO_DB
        }

        fn collection_name() -> &'static str {
            MONGO_COLLECTION
        }
    }
}

mod spec {
    use super::*;
    use crate::common::name::Name;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct ItemSpec {
        name: Name,
        size: ItemSize,
    }

    impl ItemSpec {
        pub fn new(name: Name, size: ItemSize) -> Self {
            Self { name, size }
        }

        pub fn name(&self) -> &Name {
            &self.name
        }

        pub fn name_mut(&mut self) -> &mut Name {
            &mut self.name
        }

        pub fn size(&self) -> &ItemSize {
            &self.size
        }

        pub fn size_mut(&mut self) -> &mut ItemSize {
            &mut self.size
        }
    }
}

mod patch {
    use super::*;
    use crate::{
        common::{id::Id, name::Name},
        storage::repo::Patch,
    };
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct ItemPatch {
        #[serde(skip)]
        id: Id,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<Name>,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<ItemSize>,
    }

    impl ItemPatch {
        pub fn new(id: Id) -> Self {
            Self {
                id,
                name: None,
                size: None,
            }
        }

        pub fn name(&self) -> &Option<Name> {
            &self.name
        }

        pub fn name_mut(&mut self) -> &mut Option<Name> {
            &mut self.name
        }

        pub fn size(&self) -> &Option<ItemSize> {
            &self.size
        }

        pub fn size_mut(&mut self) -> &mut Option<ItemSize> {
            &mut self.size
        }
    }

    impl Patch for ItemPatch {
        fn id(&self) -> &Id {
            &self.id
        }
    }
}

mod filter {
    use super::*;
    use crate::{
        common::{id::Id, name::Name},
        storage::repo::Filter,
    };
    use serde::Serialize;

    #[derive(Default, Serialize)]
    pub struct ItemFilter {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<Id>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<Name>,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<ItemSize>,
    }

    impl ItemFilter {
        pub fn id(&self) -> &Option<Id> {
            &self.id
        }

        pub fn name(&self) -> &Option<Name> {
            &self.name
        }

        pub fn name_mut(&mut self) -> &mut Option<Name> {
            &mut self.name
        }

        pub fn size(&self) -> &Option<ItemSize> {
            &self.size
        }

        pub fn size_mut(&mut self) -> &mut Option<ItemSize> {
            &mut self.size
        }
    }

    impl Filter for ItemFilter {
        fn id_mut(&mut self) -> &mut Option<Id> {
            &mut self.id
        }
    }
}
