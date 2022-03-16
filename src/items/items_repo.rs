use crate::items::{Item, ItemSpec};
use crate::storage::MongoRepo;

pub type MongoItemsRepo = MongoRepo<Item, ItemSpec>;
