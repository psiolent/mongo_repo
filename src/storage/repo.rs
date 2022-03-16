use crate::common::{Entity, Id};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Repo<E: Entity, S> {
    type RepoError: Error;

    async fn create(&self, spec: &S) -> Result<Id, Self::RepoError>;

    async fn update(&self, entity: &E) -> Result<bool, Self::RepoError>;

    async fn delete(&self, id: &Id) -> Result<bool, Self::RepoError>;

    async fn retrieve(&self, id: &Id) -> Result<Option<E>, Self::RepoError>;

    async fn retrieve_all(&self) -> Result<Vec<E>, Self::RepoError>;
}

pub trait RepoError: Error {}
