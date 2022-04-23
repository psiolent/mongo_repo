use crate::common::id::Id;
use crate::common::name::Name;
use crate::items::item::{Item, ItemSize};
use crate::storage::mongo_repo::{MongoRepo, MongoReposable};
use crate::storage::repo::{Filter, Patch, Reposable};
use mongodb::bson::Document;
use serde::Serialize;

const MONGO_DB: &str = "repotest";
const MONGO_COLLECTION: &str = "items";

pub type MongoItemsRepo = MongoRepo<Item>;

impl Reposable for Item {
    type Spec = ItemSpec;
    type Patch = ItemPatch;
    type Filter = ItemFilter;
}

#[derive(Serialize)]
pub struct ItemSpec {
    name: Name,
    size: ItemSize,
}

#[derive(Serialize)]
pub struct ItemPatch {
    id: Id,
    name: Option<Name>,
    size: Option<ItemSize>,
}

#[derive(Default, Serialize)]
pub struct ItemFilter {
    id: Option<Id>,
    name: Option<Name>,
    size: Option<ItemSize>,
}

impl MongoReposable for Item {
    fn db_name() -> &'static str {
        MONGO_DB
    }

    fn collection_name() -> &'static str {
        MONGO_COLLECTION
    }
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

impl ItemPatch {
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
    fn update_id(&self) -> &Id {
        &self.id
    }
}

impl From<ItemPatch> for Document {
    fn from(_: ItemPatch) -> Self {
        todo!()
    }
}

impl ItemFilter {
    pub fn id(&self) -> &Option<Id> {
        &self.id
    }

    pub fn id_mut(&mut self) -> &mut Option<Id> {
        &mut self.id
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
    fn by_id(&mut self, id: &Id) {
        self.id = Some(id.clone())
    }
}
