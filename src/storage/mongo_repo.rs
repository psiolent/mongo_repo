use crate::common::{Entity, Id};
use crate::storage::Repo;
use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::bson::{doc, Bson};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MongoRepo<E, S>
where
    E: Entity + Serialize + DeserializeOwned + Unpin + Send + Sync,
    S: Serialize + Send + Sync,
{
    db_name: String,
    collection_name: String,
    client: mongodb::Client,
    session: Option<Arc<Mutex<mongodb::ClientSession>>>,
    _entity: PhantomData<E>,
    _spec: PhantomData<S>,
}

impl<E, S> MongoRepo<E, S>
where
    E: Entity + Serialize + DeserializeOwned + Unpin + Send + Sync,
    S: Serialize + Send + Sync,
{
    pub fn new(db_name: &str, collection_name: &str, client: mongodb::Client) -> Self {
        Self {
            db_name: db_name.into(),
            collection_name: collection_name.into(),
            client,
            session: None,
            _entity: PhantomData,
            _spec: PhantomData,
        }
    }

    pub fn new_with_session(
        db_name: &str,
        collection_name: &str,
        client: mongodb::Client,
        session: Arc<Mutex<mongodb::ClientSession>>,
    ) -> Self {
        Self {
            db_name: db_name.into(),
            collection_name: collection_name.into(),
            client,
            session: Some(session),
            _entity: PhantomData,
            _spec: PhantomData,
        }
    }

    fn collection<T>(&self) -> mongodb::Collection<T> {
        self.client
            .database(self.db_name.as_str())
            .collection(self.collection_name.as_str())
    }
}

#[async_trait]
impl<E, S> Repo<E, S> for MongoRepo<E, S>
where
    E: Entity + Serialize + DeserializeOwned + Unpin + Send + Sync,
    S: Serialize + Send + Sync,
{
    type RepoError = MongoRepoError;

    async fn create(&self, spec: &S) -> Result<Id, Self::RepoError> {
        let coll = self.collection::<S>();
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

    async fn update(&self, entity: &E) -> Result<bool, Self::RepoError> {
        let query = doc! { "_id": entity.id() };
        let coll = self.collection::<E>();
        let result = match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                coll.replace_one_with_session(query, entity, None, session)
                    .await?
            }
            None => coll.replace_one(query, entity, None).await?,
        };
        Ok(result.modified_count > 0)
    }

    async fn delete(&self, id: &Id) -> Result<bool, Self::RepoError> {
        let query = doc! { "_id": id };
        let coll = self.collection::<E>();
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

    async fn retrieve(&self, id: &Id) -> Result<Option<E>, Self::RepoError> {
        let filter = doc! { "_id": id };
        let coll = self.collection::<E>();
        match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                Ok(coll.find_one_with_session(filter, None, session).await?)
            }
            None => Ok(coll.find_one(filter, None).await?),
        }
    }

    async fn retrieve_all(&self) -> Result<Vec<E>, Self::RepoError> {
        let coll = self.collection::<E>();
        match self.session {
            Some(ref session) => {
                let mut session_guard = session.lock().await;
                let session = session_guard.deref_mut();
                let mut cursor = coll.find_with_session(None, None, session).await?;
                let mut docs = vec![];
                while let Some(doc) = cursor.next(session).await {
                    docs.push(doc?);
                }
                Ok(docs)
            }
            None => {
                let mut cursor = coll.find(None, None).await?;
                let mut docs = vec![];
                while let Some(doc) = cursor.next().await {
                    docs.push(doc?);
                }
                Ok(docs)
            }
        }
    }
}

impl<E, S> Clone for MongoRepo<E, S>
where
    E: Entity + Serialize + DeserializeOwned + Unpin + Send + Sync,
    S: Serialize + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            db_name: self.db_name.clone(),
            collection_name: self.collection_name.clone(),
            client: self.client.clone(),
            session: self.session.clone(),
            _entity: self._entity,
            _spec: self._spec,
        }
    }
}

#[derive(Debug)]
pub enum MongoRepoError {
    MongoError(mongodb::error::ErrorKind),
}

impl Error for MongoRepoError {}

impl Display for MongoRepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MongoError(e) => write!(f, "MongoError({})", e),
        }
    }
}

impl From<mongodb::error::Error> for MongoRepoError {
    fn from(e: mongodb::error::Error) -> Self {
        MongoRepoError::MongoError(*e.kind)
    }
}
