use crate::domain::{Domain, DomainImpl, MongoDomainContext};

#[derive(Clone)]
pub struct ContextFactory {
    mongo_client: mongodb::Client,
}

impl ContextFactory {
    pub fn new(mongo_client: mongodb::Client) -> Self {
        Self { mongo_client }
    }

    pub fn create_context(&self) -> Context {
        Context::new(self.mongo_client.clone())
    }
}

#[derive(Clone)]
pub struct Context {
    domain: DomainImpl<MongoDomainContext>,
}

impl juniper::Context for Context {}

impl Context {
    fn new(mongo_client: mongodb::Client) -> Self {
        let domain = DomainImpl::new(MongoDomainContext::new(mongo_client));
        Context { domain }
    }

    pub fn domain(&self) -> &impl Domain {
        &self.domain
    }
}
