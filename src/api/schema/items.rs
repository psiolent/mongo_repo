use crate::{
    api::context::Context,
    common::{entity::Entity, id::Id},
    items::{self, item::Item},
    storage::repo::Repo,
};
use juniper::{graphql_object, FieldResult};
use std::str::FromStr;

pub struct ItemNode(Item);

#[derive(juniper::GraphQLEnum)]
#[graphql(description = "The size of an item")]
pub enum ItemSize {
    #[graphql(description = "The smallest of sizes")]
    Small,
    #[graphql(description = "A medium size")]
    Medium,
    #[graphql(description = "Large and in charge")]
    Large,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "Input for creating or updating an item")]
pub struct ItemInput {
    #[graphql(description = "The name of the item")]
    pub name: String,
    pub size: ItemSize,
}

#[graphql_object(context = Context)]
#[graphql(name = "Item", description = "A named item with a specific size")]
impl ItemNode {
    #[graphql(description = "The unique identifier for the item")]
    pub fn id(&self) -> String {
        self.0.id().to_string()
    }

    #[graphql(description = "The name of the item")]
    pub fn name(&self) -> &str {
        self.0.name()
    }

    #[graphql(description = "The size of the item")]
    pub fn size(&self) -> ItemSize {
        self.0.size().into()
    }
}

impl From<Item> for ItemNode {
    fn from(item: Item) -> Self {
        Self(item)
    }
}

impl From<&items::item::ItemSize> for ItemSize {
    fn from(size: &items::item::ItemSize) -> Self {
        match size {
            items::item::ItemSize::Small => ItemSize::Small,
            items::item::ItemSize::Medium => ItemSize::Medium,
            items::item::ItemSize::Large => ItemSize::Large,
        }
    }
}

pub async fn item_nodes(ctx: &Context) -> FieldResult<Vec<ItemNode>> {
    let items = ctx
        .items_repo()
        .retrieve_page(0, 0)
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
