use crate::api::{Context, Mutation, Query};
use crate::items::MongoItemsRepo;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Response, Server, StatusCode};
use juniper::{EmptySubscription, RootNode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct ApiServer {
    pub bind_addr: SocketAddr,
}

impl ApiServer {
    pub async fn run(&self) {
        let mut client_options =
            mongodb::options::ClientOptions::parse("mongodb://127.0.0.1:27017")
                .await
                .unwrap();
        client_options.app_name = Some("Mongo+GraphQL Test".to_string());
        let client = mongodb::Client::with_options(client_options).unwrap();
        let items_repo = MongoItemsRepo::new("test", "items", client);

        let ctx = Arc::new(Context {
            items_repo,
            api_base_path: "http://192.168.6.6:4000".into(),
        });
        let root_node = Arc::new(RootNode::new(
            Query,
            Mutation,
            EmptySubscription::<Context>::new(),
        ));

        let make_svc = make_service_fn(move |_| {
            let ctx = ctx.clone();
            let root_node = root_node.clone();

            async {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let ctx = ctx.clone();
                    let root_node = root_node.clone();
                    async {
                        Ok::<_, Infallible>(match (req.method(), req.uri().path()) {
                            (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
                            (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                                juniper_hyper::graphql(root_node, ctx, req).await
                            }
                            _ => {
                                let mut response = Response::new(Body::empty());
                                *response.status_mut() = StatusCode::NOT_FOUND;
                                response
                            }
                        })
                    }
                }))
            }
        });

        let server = Server::bind(&self.bind_addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}
