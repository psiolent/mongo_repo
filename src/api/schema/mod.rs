pub mod items;

use crate::api::{
    context::Context,
    schema::items::{item_node, item_nodes, ItemNode},
};
use juniper::{graphql_object, FieldResult};

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
}

#[derive(Clone)]
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn foo() -> FieldResult<String> {
        Ok("not yet implemented".into())
    }
}
