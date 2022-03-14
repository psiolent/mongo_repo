use crate::common::{Entity, Id};
use crate::items::Item;
use crate::storage::Repository;
use async_trait::async_trait;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

const MONGO_DB: &str = "test";
const MONGO_COLLECTION: &str = "items";

trait ItemsRepo: Repository<Item> {}

#[derive(Clone)]
pub struct MongoItemsRepo {
    client: mongodb::Client,
    session: Option<Arc<Mutex<mongodb::ClientSession>>>,
}

impl MongoItemsRepo {
    pub fn new(client: mongodb::Client) -> Self {
        Self {
            client,
            session: None,
        }
    }

    pub fn new_with_session(
        client: mongodb::Client,
        session: Arc<Mutex<mongodb::ClientSession>>,
    ) -> Self {
        Self {
            client,
            session: Some(session),
        }
    }

    fn collection(&self) -> mongodb::Collection<ItemDoc> {
        self.client.database(MONGO_DB).collection(MONGO_COLLECTION)
    }
}

#[async_trait]
impl Repository<Item> for MongoItemsRepo {
    type RepoError = mongodb::error::ErrorKind;

    async fn insert(&self, entity: &Item) -> Result<bool, Self::RepoError> {
        let doc = ItemDoc::new(entity);
        let coll = self.collection();
        let insert_result = match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                coll.insert_one_with_session(doc, None, session).await
            }
            None => coll.insert_one(doc, None).await,
        };
        insert_result.map_err(|e| *e.kind)?;
        Ok(true)
    }

    async fn update(&self, entity: &Item) -> Result<bool, Self::RepoError> {
        todo!()
    }

    async fn delete(&self, id: &Id) -> Result<bool, Self::RepoError> {
        todo!()
    }

    async fn retrieve(&self, id: &Id) -> Result<Option<Item>, Self::RepoError> {
        todo!()
    }

    async fn retrieve_page(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Item>, Self::RepoError> {
        todo!()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ItemDoc {
    #[serde(rename = "_id")]
    id: Id,
    name: String,
}

impl ItemDoc {
    fn new(item: &Item) -> Self {
        ItemDoc {
            id: item.id().clone(),
            name: item.name().into(),
        }
    }
}

impl From<ItemDoc> for Item {
    fn from(doc: ItemDoc) -> Item {
        Item::new(doc.id, &doc.name)
    }
}
