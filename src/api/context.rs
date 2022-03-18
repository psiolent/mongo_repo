use ensure_config_rust_client::apis::configuration::Configuration;

use crate::items::MongoItemsRepo;

/*
pub struct ContextFactory {
    mongo_client: mongodb::Client,
}

impl ContextFactory {
    pub fn new(mongo_client: mongodb::Client) -> Self {
        Self { mongo_client }
    }

    pub async fn create_context() -> Context {

    }
}
*/

#[derive(Clone)]
pub struct Context {
    pub items_repo: MongoItemsRepo,
    pub api_base_path: String,
}

impl juniper::Context for Context {}

impl Context {
    pub fn items_repo(&self) -> &MongoItemsRepo {
        &self.items_repo
    }

    pub fn api_config(&self) -> Configuration {
        Configuration {
            base_path: self.api_base_path.clone(),
            ..Default::default()
        }
    }
}
