pub mod models;

pub use context::*;

use self::models::items::{Item, ItemFilter, ItemPatch, ItemSpec};
use crate::{
    common::id::Id,
    storage::repo::{Patch, Repo},
};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Domain {
    type DomainError: Error;

    async fn item(&self, id: &Id) -> Result<Option<Item>, Self::DomainError>;
    async fn all_items(&self) -> Result<Vec<Item>, Self::DomainError>;
    async fn find_items(&self, filter: &ItemFilter) -> Result<Vec<Item>, Self::DomainError>;
    async fn create_item(&self, spec: &ItemSpec) -> Result<Item, Self::DomainError>;
    async fn update_item(&self, patch: &ItemPatch) -> Result<Option<Item>, Self::DomainError>;
    async fn delete_item(&self, id: &Id) -> Result<bool, Self::DomainError>;
}

#[derive(Clone)]
pub struct DomainImpl<C: DomainContext> {
    ctx: C,
}

impl<C: DomainContext> DomainImpl<C> {
    pub fn new(ctx: C) -> Self {
        Self { ctx }
    }
}

#[async_trait]
impl<C: DomainContext> Domain for DomainImpl<C>
where
    C: Send + Sync,
    C::ItemsRepo: Sync,
    <C::ItemsRepo as Repo<Item>>::RepoError: Send,
{
    // FIXME: this is a hack...what should the error type be?
    type DomainError = <C::ItemsRepo as Repo<Item>>::RepoError;

    async fn item(&self, id: &Id) -> Result<Option<Item>, Self::DomainError> {
        self.ctx.items_repo().retrieve(id).await
    }

    async fn all_items(&self) -> Result<Vec<Item>, Self::DomainError> {
        self.ctx.items_repo().retrieve_all().await
    }

    async fn find_items(&self, filter: &ItemFilter) -> Result<Vec<Item>, Self::DomainError> {
        self.ctx.items_repo().find_all(filter).await
    }

    async fn create_item(&self, spec: &ItemSpec) -> Result<Item, Self::DomainError> {
        let ctx = self.ctx.start_transaction().await;
        let items_repo = ctx.items_repo();
        let id = items_repo.create(spec).await?;
        if let Some(item) = items_repo.retrieve(&id).await? {
            ctx.commit_transaction().await;
            Ok(item)
        } else {
            panic!("item could not be retrieved following creation");
        }
    }

    async fn update_item(&self, patch: &ItemPatch) -> Result<Option<Item>, Self::DomainError> {
        let ctx = self.ctx.start_transaction().await;
        let items_repo = ctx.items_repo();
        match items_repo.update(patch).await? {
            true => {
                if let Some(item) = items_repo.retrieve(patch.id()).await? {
                    ctx.commit_transaction().await;
                    Ok(Some(item))
                } else {
                    panic!("item could not be retrieved following update");
                }
            }
            false => Ok(None),
        }
    }

    async fn delete_item(&self, id: &Id) -> Result<bool, Self::DomainError> {
        let ctx = self.ctx.start_transaction().await;
        let items_repo = ctx.items_repo();
        match items_repo.delete(id).await? {
            true => {
                ctx.commit_transaction().await;
                Ok(true)
            }
            false => Ok(false),
        }
    }
}

mod context {
    use super::models::items::Item;
    use crate::storage::{mongo_repo::MongoRepo, repo::Repo};
    use async_trait::async_trait;
    use std::{ops::DerefMut, sync::Arc};
    use tokio::sync::Mutex;

    #[async_trait]
    pub trait DomainContext: Clone {
        type ItemsRepo: Repo<Item>;

        fn items_repo(&self) -> &Self::ItemsRepo;

        async fn start_transaction(&self) -> Self;
        async fn abort_transaction(mut self);
        async fn commit_transaction(mut self);
    }

    #[derive(Clone)]
    pub struct MongoDomainContext {
        mongo_client: mongodb::Client,
        mongo_session: Option<Arc<Mutex<mongodb::ClientSession>>>,
        items_repo: MongoRepo<Item>,
    }

    impl MongoDomainContext {
        pub fn new(mongo_client: mongodb::Client) -> Self {
            let mongo_session = None;
            let items_repo = MongoRepo::new(mongo_client.clone());
            Self {
                mongo_client,
                mongo_session,
                items_repo,
            }
        }
    }

    #[async_trait]
    impl DomainContext for MongoDomainContext {
        type ItemsRepo = MongoRepo<Item>;

        fn items_repo(&self) -> &MongoRepo<Item> {
            &self.items_repo
        }

        async fn start_transaction(&self) -> Self {
            let mongo_client = self.mongo_client.clone();
            let mut mongo_session = mongo_client.start_session(None).await.unwrap();
            mongo_session.start_transaction(None).await.unwrap();
            let mongo_session = Arc::new(tokio::sync::Mutex::new(mongo_session));
            let items_repo =
                MongoRepo::new_with_session(mongo_client.clone(), Arc::clone(&mongo_session));
            let mongo_session = Some(mongo_session);
            Self {
                mongo_client,
                mongo_session,
                items_repo,
            }
        }

        async fn abort_transaction(mut self) {
            let session = self.mongo_session.take().unwrap();
            let mut session_guard = session.lock().await;
            let session = session_guard.deref_mut();
            session.abort_transaction().await.unwrap();
        }

        async fn commit_transaction(mut self) {
            let session = self.mongo_session.take().unwrap();
            let mut session_guard = session.lock().await;
            let session = session_guard.deref_mut();
            session.commit_transaction().await.unwrap();
        }
    }
}
