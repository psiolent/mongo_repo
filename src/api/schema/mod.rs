pub mod items;

use crate::api::{
    context::Context,
    schema::items::{
        create_item_from_input, delete_item_by_id, item_node, item_nodes, update_item_from_input,
        CreateItemInput, ItemFilterInput, ItemNode, UpdateItemInput,
    },
};
use juniper::{graphql_object, FieldResult};

#[derive(Clone)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn items(ctx: &Context, filter: Option<ItemFilterInput>) -> FieldResult<Vec<ItemNode>> {
        item_nodes(ctx, filter).await
    }

    async fn item(ctx: &Context, id: String) -> FieldResult<Option<ItemNode>> {
        item_node(ctx, id).await
    }
}

#[derive(Clone)]
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_item(ctx: &Context, input: CreateItemInput) -> FieldResult<ItemNode> {
        create_item_from_input(ctx, input).await
    }

    async fn update_item(ctx: &Context, input: UpdateItemInput) -> FieldResult<ItemNode> {
        update_item_from_input(ctx, input).await
    }

    async fn delete_item(ctx: &Context, id: String) -> FieldResult<String> {
        delete_item_by_id(ctx, id.as_str()).await?;
        Ok(id)
    }
}
