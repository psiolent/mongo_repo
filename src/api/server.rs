use crate::api::{
    context::{Context, ContextFactory},
    schema::{Mutation, Query},
};
use futures::Future;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Response, Server, StatusCode,
};
use juniper::{EmptySubscription, RootNode};
use log::{debug, error, info};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

pub const DEFAULT_BIND_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const DEFAULT_BIND_PORT: u16 = 3000;

pub async fn run_api_server(
    bind_ip_addr: IpAddr,
    bind_port: u16,
    mongo_client: mongodb::Client,
    shutdown_signal: impl Future<Output = ()>,
) {
    info!("starting api server");

    let ctx_factory = ContextFactory::new(mongo_client);
    let root_node = Arc::new(RootNode::new(
        Query,
        Mutation,
        EmptySubscription::<Context>::new(),
    ));

    let make_svc = make_service_fn(move |_| {
        let ctx_factory = ctx_factory.clone();
        let root_node = root_node.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                debug!("{} {} {:?}", req.method(), req.uri(), req.version());
                let ctx = ctx_factory.create_context();
                let root_node = root_node.clone();
                async move {
                    Ok::<_, Infallible>(match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(root_node, Arc::new(ctx), req).await
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

    let server = Server::bind(&(bind_ip_addr, bind_port).into())
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    info!("stopped");
}
