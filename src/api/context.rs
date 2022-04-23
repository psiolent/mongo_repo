use std::{ops::DerefMut, sync::Arc};
use tokio::sync::Mutex;

use crate::items::item_repo::MongoItemsRepo;

#[derive(Clone)]
pub struct ContextFactory {
    mongo_client: mongodb::Client,
}

impl ContextFactory {
    pub fn new(mongo_client: mongodb::Client) -> Self {
        Self { mongo_client }
    }

    pub fn create_context(&self) -> Context {
        Context::new(self.mongo_client.clone())
    }
}

#[derive(Clone)]
pub struct Context {
    mongo_client: mongodb::Client,
    mongo_session: Option<Arc<Mutex<mongodb::ClientSession>>>,
    items_repo: MongoItemsRepo,
}

impl juniper::Context for Context {}

impl Context {
    fn new(mongo_client: mongodb::Client) -> Self {
        let mongo_session = None;
        let items_repo = MongoItemsRepo::new(mongo_client.clone());
        Context {
            mongo_client,
            mongo_session,
            items_repo,
        }
    }

    pub fn items_repo(&self) -> &MongoItemsRepo {
        &self.items_repo
    }

    pub async fn start_transaction(&self) -> Self {
        let mongo_client = self.mongo_client.clone();
        let mut mongo_session = mongo_client.start_session(None).await.unwrap();
        mongo_session.start_transaction(None).await.unwrap();
        let mongo_session = Arc::new(tokio::sync::Mutex::new(mongo_session));
        let items_repo =
            MongoItemsRepo::new_with_session(mongo_client.clone(), Arc::clone(&mongo_session));
        let mongo_session = Some(mongo_session);
        Context {
            mongo_client,
            mongo_session,
            items_repo,
        }
    }

    pub async fn abort_transaction(mut self) {
        let session = self.mongo_session.take().unwrap();
        let mut session_guard = session.lock().await;
        let session = session_guard.deref_mut();
        session.abort_transaction().await.unwrap();
    }

    pub async fn commit_transaction(mut self) {
        let session = self.mongo_session.take().unwrap();
        let mut session_guard = session.lock().await;
        let session = session_guard.deref_mut();
        session.commit_transaction().await.unwrap();
    }
}
