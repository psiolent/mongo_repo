pub mod items;
pub mod locations;

use crate::api::items::{item_node, item_nodes, ItemNode};
use crate::api::locations::{location_node, location_nodes, LocationNode};
use crate::common::Id;
use crate::items::Item;
use crate::storage::Repo;
use crate::{api::Context, items::ItemSpec};
use juniper::{graphql_object, FieldResult};
use std::str::FromStr;

#[derive(Clone)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn items(ctx: &Context) -> FieldResult<Vec<ItemNode>> {
        item_nodes(ctx).await
    }

    async fn item(ctx: &Context, id: String) -> FieldResult<Option<ItemNode>> {
        item_node(ctx, id).await
    }

    async fn locations(ctx: &Context) -> FieldResult<Vec<LocationNode>> {
        location_nodes(ctx).await
    }

    async fn location(ctx: &Context, id: String) -> FieldResult<Option<LocationNode>> {
        location_node(ctx, id).await
    }
}

#[derive(Clone)]
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_item(ctx: &Context, name: String) -> FieldResult<String> {
        let spec = ItemSpec { name };
        let new_id = ctx.items_repo().create(&spec).await?;
        Ok(new_id.to_string())
    }

    async fn update_item(ctx: &Context, id: String, name: String) -> FieldResult<bool> {
        let id = Id::from_str(&id)?;
        let item = Item::new(id, name);
        let result = ctx.items_repo.update(&item).await?;
        Ok(result)
    }

    async fn delete_item(ctx: &Context, id: String) -> FieldResult<bool> {
        let id = Id::from_str(&id)?;
        let result = ctx.items_repo.delete(&id).await?;
        Ok(result)
    }
}
