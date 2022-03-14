use crate::common::Id;

pub trait Entity {
    fn id(&self) -> &Id;
}
