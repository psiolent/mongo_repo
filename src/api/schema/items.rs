pub use controller::*;
pub use create::*;
pub use find::*;
pub use node::*;
pub use update::*;

mod controller {
    use super::*;
    use crate::{
        api::context::Context,
        common::id::Id,
        domain::models::items::{ItemFilter, ItemPatch, ItemSpec},
        domain::Domain,
    };
    use juniper::FieldResult;
    use std::str::FromStr;

    pub async fn create_item_from_input(
        ctx: &Context,
        input: CreateItemInput,
    ) -> FieldResult<ItemNode> {
        let spec = ItemSpec::try_from(input)?;
        Ok(ItemNode::from(ctx.domain().create_item(&spec).await?))
    }

    pub async fn update_item_from_input(
        ctx: &Context,
        input: UpdateItemInput,
    ) -> FieldResult<ItemNode> {
        let patch = ItemPatch::try_from(input)?;
        if let Some(item) = ctx.domain().update_item(&patch).await? {
            Ok(ItemNode::from(item))
        } else {
            Err(String::from("no item exists with the provided ID").into())
        }
    }

    pub async fn delete_item_by_id(ctx: &Context, id: &str) -> FieldResult<()> {
        let id = id
            .parse::<Id>()
            .map_err(|_| String::from("the provided ID was invalid"))?;
        match ctx.domain().delete_item(&id).await? {
            true => Ok(()),
            false => Err(String::from("no item exists with the provided ID").into()),
        }
    }

    pub async fn item_nodes(
        ctx: &Context,
        filter: Option<ItemFilterInput>,
    ) -> FieldResult<Vec<ItemNode>> {
        let items = if let Some(filter) = filter {
            ctx.domain()
                .find_items(&ItemFilter::try_from(filter)?)
                .await?
        } else {
            ctx.domain().all_items().await?
        };
        let item_nodes = items.into_iter().map(ItemNode::from).collect();
        Ok(item_nodes)
    }

    pub async fn item_node(ctx: &Context, id: String) -> FieldResult<Option<ItemNode>> {
        let id = Id::from_str(&id)?;
        let item = ctx.domain().item(&id).await?;
        Ok(item.map(ItemNode::from))
    }
}

mod node {
    use crate::{
        api::context::Context,
        common::entity::Entity,
        domain::models::items::{self, Item},
    };
    use juniper::graphql_object;

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

    impl From<&items::ItemSize> for ItemSize {
        fn from(size: &items::ItemSize) -> Self {
            match size {
                items::ItemSize::Small => ItemSize::Small,
                items::ItemSize::Medium => ItemSize::Medium,
                items::ItemSize::Large => ItemSize::Large,
            }
        }
    }

    impl From<&ItemSize> for items::ItemSize {
        fn from(size: &ItemSize) -> Self {
            match size {
                ItemSize::Small => items::ItemSize::Small,
                ItemSize::Medium => items::ItemSize::Medium,
                ItemSize::Large => items::ItemSize::Large,
            }
        }
    }
}

mod create {
    use super::ItemSize;
    use crate::{
        common::name::Name,
        domain::models::items::{self, ItemSpec},
    };

    #[derive(juniper::GraphQLInputObject)]
    #[graphql(description = "Input for creating an item")]
    pub struct CreateItemInput {
        #[graphql(description = "The name of the item to create")]
        pub name: String,
        #[graphql(description = "The size of the item to create")]
        pub size: ItemSize,
    }

    impl TryFrom<CreateItemInput> for ItemSpec {
        type Error = String;

        fn try_from(input: CreateItemInput) -> Result<Self, Self::Error> {
            match input.name.parse::<Name>() {
                Ok(name) => Ok(ItemSpec::new(name, items::ItemSize::from(&input.size))),
                Err(_) => Err("name cannot be empty".into()),
            }
        }
    }
}

mod update {
    use crate::{
        common::{id::Id, name::Name},
        domain::models::items::ItemPatch,
    };

    use super::ItemSize;

    #[derive(juniper::GraphQLInputObject)]
    #[graphql(description = "Input for updating an item")]
    pub struct UpdateItemInput {
        pub id: String,
        pub name: Option<String>,
        pub size: Option<ItemSize>,
    }

    impl TryFrom<UpdateItemInput> for ItemPatch {
        type Error = String;

        fn try_from(input: UpdateItemInput) -> Result<Self, Self::Error> {
            let id = input
                .id
                .parse::<Id>()
                .map_err(|_| String::from("the provided ID was invalid"))?;
            let mut patch = ItemPatch::new(id);

            if let Some(s) = input.name.as_ref() {
                *patch.name_mut() = Some(
                    s.parse::<Name>()
                        .map_err(|_| String::from("cannot search for an empty name"))?,
                );
            }

            if let Some(size) = input.size.as_ref() {
                *patch.size_mut() = Some(size.into());
            }

            Ok(patch)
        }
    }
}

mod find {
    use super::ItemSize;
    use crate::{
        common::{id::Id, name::Name},
        domain::models::items::ItemFilter,
        storage::repo::Filter,
    };

    #[derive(juniper::GraphQLInputObject)]
    #[graphql(description = "Input for finding items")]
    pub struct ItemFilterInput {
        pub id: Option<String>,
        pub name: Option<String>,
        pub size: Option<ItemSize>,
    }

    impl TryFrom<ItemFilterInput> for ItemFilter {
        type Error = String;

        fn try_from(input: ItemFilterInput) -> Result<Self, Self::Error> {
            let mut filter = ItemFilter::default();

            if let Some(s) = input.id.as_ref() {
                *filter.id_mut() = Some(
                    s.parse::<Id>()
                        .map_err(|_| String::from("the provided ID was invalid"))?,
                );
            }

            if let Some(s) = input.name.as_ref() {
                *filter.name_mut() = Some(
                    s.parse::<Name>()
                        .map_err(|_| String::from("cannot search for an empty name"))?,
                );
            }

            if let Some(size) = input.size.as_ref() {
                *filter.size_mut() = Some(size.into());
            }

            Ok(filter)
        }
    }
}
