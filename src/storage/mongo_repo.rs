use crate::common::id::Id;
use crate::storage::repo::{Patch, Repo};
use async_trait::async_trait;
use futures::StreamExt;
use mongodb::bson::{doc, ser::to_document, Bson};
use mongodb::options::FindOptions;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::repo::{Filter, Reposable};

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 27017;

pub struct MongoRepo<R: MongoReposable>
where
    R: DeserializeOwned,
    R::Spec: Serialize,
    R::Patch: Serialize,
    R::Filter: Serialize,
{
    client: mongodb::Client,
    session: Option<Arc<Mutex<mongodb::ClientSession>>>,
    _reposable: PhantomData<R>,
}

pub trait MongoReposable: Reposable
where
    Self: DeserializeOwned,
    Self::Spec: Serialize,
    Self::Patch: Serialize,
    Self::Filter: Serialize,
{
    fn db_name() -> &'static str;
    fn collection_name() -> &'static str;
}

impl<R: MongoReposable> MongoRepo<R>
where
    R: DeserializeOwned,
    R::Spec: Serialize,
    R::Patch: Serialize,
    R::Filter: Serialize,
{
    pub fn new(client: mongodb::Client) -> Self {
        Self {
            client,
            session: None,
            _reposable: PhantomData,
        }
    }

    pub fn new_with_session(
        client: mongodb::Client,
        session: Arc<Mutex<mongodb::ClientSession>>,
    ) -> Self {
        Self {
            client,
            session: Some(session),
            _reposable: PhantomData,
        }
    }

    fn collection<T>(&self) -> mongodb::Collection<T> {
        self.client
            .database(R::db_name())
            .collection(R::collection_name())
    }
}

#[async_trait]
impl<R: MongoReposable> Repo<R> for MongoRepo<R>
where
    R: DeserializeOwned + Send + Sync + Unpin,
    R::Spec: Serialize + Send + Sync,
    R::Patch: Serialize + Send + Sync,
    R::Filter: Serialize + Send + Sync,
{
    type RepoError = MongoRepoError;

    async fn create(&self, spec: &R::Spec) -> Result<Id, Self::RepoError> {
        let coll = self.collection::<R::Spec>();

        let result = match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                coll.insert_one_with_session(spec, None, session).await?
            }
            None => coll.insert_one(spec, None).await?,
        };

        match result.inserted_id {
            Bson::ObjectId(oid) => Ok(oid.into()),
            _ => panic!("inserted ID was not an ObjectId"),
        }
    }

    async fn update(&self, patch: &R::Patch) -> Result<bool, Self::RepoError> {
        let mut query = R::Filter::default();
        *query.id_mut() = Some(patch.id().clone());
        let query = to_document(&query)?;
        let update = doc! { "$set": to_document(patch)? };
        let coll = self.collection::<R>();

        let result = match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                coll.update_one_with_session(query, update, None, session)
                    .await?
            }
            None => coll.update_one(query, update, None).await?,
        };

        Ok(result.modified_count > 0)
    }

    async fn delete(&self, id: &Id) -> Result<bool, Self::RepoError> {
        let mut query = R::Filter::default();
        *query.id_mut() = Some(id.clone());
        let query = to_document(&query)?;
        let coll = self.collection::<R>();

        let result = match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                coll.delete_one_with_session(query, None, session).await?
            }
            None => coll.delete_one(query, None).await?,
        };

        Ok(result.deleted_count > 0)
    }

    async fn retrieve(&self, id: &Id) -> Result<Option<R>, Self::RepoError> {
        let mut filter = R::Filter::default();
        *filter.id_mut() = Some(id.clone());
        let filter = to_document(&filter)?;
        let coll = self.collection::<R>();

        match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                Ok(coll.find_one_with_session(filter, None, session).await?)
            }
            None => Ok(coll.find_one(filter, None).await?),
        }
    }

    async fn retrieve_all(&self) -> Result<Vec<R>, Self::RepoError> {
        self.find_all(&R::Filter::default()).await
    }

    async fn retrieve_page(&self, offset: usize, limit: usize) -> Result<Vec<R>, Self::RepoError> {
        self.find_page(&R::Filter::default(), offset, limit).await
    }

    async fn find_all(&self, filter: &R::Filter) -> Result<Vec<R>, Self::RepoError> {
        let filter = to_document(filter)?;
        let coll = self.collection::<R>();

        match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                let mut cursor = coll.find_with_session(filter, None, session).await?;
                let mut docs = vec![];
                while let Some(doc) = cursor.next(session).await {
                    docs.push(doc?);
                }
                Ok(docs)
            }
            None => {
                let mut cursor = coll.find(filter, None).await?;
                let mut docs = vec![];
                while let Some(doc) = cursor.next().await {
                    docs.push(doc?);
                }
                Ok(docs)
            }
        }
    }

    async fn find_page(
        &self,
        filter: &R::Filter,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<R>, Self::RepoError> {
        let filter = to_document(filter)?;
        let options = FindOptions::builder()
            .skip(offset as u64)
            .limit(limit as i64)
            .build();
        let coll = self.collection::<R>();

        match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                let mut cursor = coll.find_with_session(filter, options, session).await?;
                let mut docs = vec![];
                while let Some(doc) = cursor.next(session).await {
                    docs.push(doc?);
                }
                Ok(docs)
            }
            None => {
                let mut cursor = coll.find(filter, options).await?;
                let mut docs = vec![];
                while let Some(doc) = cursor.next().await {
                    docs.push(doc?);
                }
                Ok(docs)
            }
        }
    }
}

impl<R: MongoReposable> Clone for MongoRepo<R>
where
    R: DeserializeOwned,
    R::Spec: Serialize,
    R::Patch: Serialize,
    R::Filter: Serialize,
{
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            session: self.session.clone(),
            _reposable: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum MongoRepoError {
    MongoError(mongodb::error::ErrorKind),
    BsonSerError(mongodb::bson::ser::Error),
}

impl Error for MongoRepoError {}

impl Display for MongoRepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MongoError(e) => write!(f, "MongoError({})", e),
            Self::BsonSerError(e) => write!(f, "BsonSerError({})", e),
        }
    }
}

impl From<mongodb::error::Error> for MongoRepoError {
    fn from(e: mongodb::error::Error) -> Self {
        MongoRepoError::MongoError(*e.kind)
    }
}

impl From<mongodb::bson::ser::Error> for MongoRepoError {
    fn from(e: mongodb::bson::ser::Error) -> Self {
        MongoRepoError::BsonSerError(e)
    }
}
