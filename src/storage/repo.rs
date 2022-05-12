use crate::common::{entity::Entity, id::Id};
use async_trait::async_trait;
use std::error::Error;

/// Defines an interface for repositories, i.e. collections of entities that allows
/// creation, updating, deletion and retrieval.
#[async_trait]
pub trait Repo<R: Reposable> {
    /// The type of errors produced by the repository.
    type RepoError: Error;

    /// Creates a new entity in the repository.
    ///
    /// # Arguments
    /// * `spec` - a specification for the entity to create
    ///
    /// # Returns
    /// the ID of the newly created entity
    async fn create(&self, spec: &R::Spec) -> Result<Id, Self::RepoError>;

    /// Updates an entity in the repository if it exists.
    ///
    /// # Arguments
    /// * `patch` - the patch to use to update the entity
    ///
    /// # Returns
    /// `true` if the entity existed and was updated, `false` otherwise
    async fn update(&self, patch: &R::Patch) -> Result<bool, Self::RepoError>;

    /// Deletes an entity from the repository if it exists.
    ///
    /// # Arguments
    /// * `id` - the ID of the entity to delete
    ///
    /// # Returns
    /// `true` if the entity existed and was deleted, `false` otherwise
    async fn delete(&self, id: &Id) -> Result<bool, Self::RepoError>;

    /// Retrieves an entity from the repository.
    ///
    /// # Arguments
    /// * `id` - the ID of the entity to retrieve
    ///
    /// # Returns
    /// `Some()` of the entity if it existed, `None` otherwise
    async fn retrieve(&self, id: &Id) -> Result<Option<R>, Self::RepoError>;

    /// Retrieves all entities from the repository.
    ///
    /// # Returns
    /// a `Vec` of all entities in the repository
    async fn retrieve_all(&self) -> Result<Vec<R>, Self::RepoError>;

    /// Retrieves a page of entities from the repository.
    ///
    /// # Arguments
    /// * `offset` - where in the entity collection to start retrieving from; the first entity is at offset `0`
    /// * `limit` - the size of the page to retrieve, i.e. the maximum number of entities to return
    ///
    /// # Returns
    /// a `Vec` of entities starting at `offset`, with a maximum of `limit` entities; if the returned vector is
    /// empty, that indicates `offset` was larger than the number of entities in the repository
    async fn retrieve_page(&self, offset: usize, limit: usize) -> Result<Vec<R>, Self::RepoError>;

    /// Retrieves all entities from the repository that match the given filter.
    ///
    /// # Arguments
    /// * `filter` - the filter to use to find matching entities
    ///
    /// # Returns
    /// a `Vec` of matching entities in the repository
    async fn find_all(&self, filter: &R::Filter) -> Result<Vec<R>, Self::RepoError>;

    /// Retrieves a page of entities from the repository that match the given filter.
    ///
    /// # Arguments
    /// * `filter` - the filter to use to find matching entities
    /// * `offset` - where in the entity collection to start retrieving from; the first entity is at offset `0`
    /// * `limit` - the size of the page to retrieve, i.e. the maximum number of entities to return
    ///
    /// # Returns
    /// a `Vec` of matching entities starting at `offset`, with a maximum of `limit` entities; if the returned
    /// vector is empty, that indicates `offset` was larger than the number of matching entities in the repository
    async fn find_page(
        &self,
        filter: &R::Filter,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<R>, Self::RepoError>;
}

/// A thing that can be reposed in a repository.
pub trait Reposable: Entity {
    type Spec;
    type Patch: Patch;
    type Filter: Filter;
}

/// A thing that can patch update a reposable thing stored in a repository.
pub trait Patch {
    /// Returns the ID of the thing for which this patch is an update.
    fn id(&self) -> &Id;
}

/// A thing that can filter reposable things stored in a repository.
pub trait Filter: Default {
    /// Modifies this filter to filter for the thing or things with the provided identity.
    fn id_mut(&mut self) -> &mut Option<Id>;
}
