use crate::common::id::Id;

/// An `Entity` is a thing that can be uniquely identified.
pub trait Entity {
    /// Returns the ID of the entity.
    fn id(&self) -> &Id;
}
