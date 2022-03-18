use std::str::FromStr;

use crate::common::{Entity, Id};
use crate::storage::Repo;
use crate::{api::Context, items::Item};
use juniper::{graphql_object, FieldResult};

pub struct ItemNode(Item);

impl From<Item> for ItemNode {
    fn from(item: Item) -> Self {
        Self(item)
    }
}

#[graphql_object(context = Context)]
#[graphql(name = "Item", description = "A named item")]
impl ItemNode {
    #[graphql(description = "The unique identifier for the item")]
    pub fn id(&self) -> String {
        self.0.id().to_string()
    }

    #[graphql(description = "The name of the item")]
    pub fn name(&self) -> &str {
        self.0.name()
    }
}

pub async fn item_nodes(ctx: &Context) -> FieldResult<Vec<ItemNode>> {
    let items = ctx
        .items_repo()
        .retrieve_all()
        .await?
        .into_iter()
        .map(ItemNode::from)
        .collect();
    Ok(items)
}

pub async fn item_node(ctx: &Context, id: String) -> FieldResult<Option<ItemNode>> {
    let id = Id::from_str(&id)?;
    let item = ctx.items_repo().retrieve(&id).await?;
    Ok(item.map(ItemNode::from))
}
