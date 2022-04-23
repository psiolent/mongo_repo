use ::mongo_repo::{
    api::{self, server::run_api_server},
    storage::mongo_repo::{self},
};
use futures::Future;
use log::{error, info};
use std::{env, net::IpAddr};
use tokio::{sync::oneshot, task::JoinHandle, try_join};

const MONGO_HOST_ENV_KEY: &str = "MONGO_HOST";
const MONGO_PORT_ENV_KEY: &str = "MONGO_PORT";
const API_BIND_IP_ENV_KEY: &str = "API_BIND_IP";
const API_BIND_PORT_ENV_KEY: &str = "API_BIND_PORT";

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx_shutdown, rx_shutdown) = oneshot::channel::<()>();
    let shutdown_signal = async move {
        rx_shutdown.await.ok();
    };
    let api_server_handle = start_api_server(shutdown_signal);

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for Ctrl-C");

    info!("got Ctrl-C; sending shutdown signal");

    tx_shutdown.send(()).ok();

    match try_join!(api_server_handle) {
        Ok(_) => (),
        Err(e) => {
            error!("task join error: {:?}", e);
        }
    }
}

fn start_api_server(shutdown_signal: impl Future<Output = ()> + Send + 'static) -> JoinHandle<()> {
    // get mongo info
    let mongo_host =
        env::var(MONGO_HOST_ENV_KEY).unwrap_or_else(|_| mongo_repo::DEFAULT_HOST.into());
    let mongo_port = env::var(MONGO_PORT_ENV_KEY)
        .map(|mongo_port_string| {
            mongo_port_string
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("invalid mongo port: {mongo_port_string}"))
        })
        .unwrap_or(mongo_repo::DEFAULT_PORT);
    let mongo_connect_string = format!("mongodb://{mongo_host}:{mongo_port}");

    // get server bind info
    let server_bind_ip = env::var(API_BIND_IP_ENV_KEY)
        .map(|bind_ip_string| {
            bind_ip_string
                .parse::<IpAddr>()
                .unwrap_or_else(|_| panic!("invalid bind IP address: {bind_ip_string}"))
        })
        .unwrap_or(api::server::DEFAULT_BIND_IP);
    let server_bind_port = env::var(API_BIND_PORT_ENV_KEY)
        .map(|bind_port_string| {
            bind_port_string
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("invalid bind port: {bind_port_string}"))
        })
        .unwrap_or(api::server::DEFAULT_BIND_PORT);

    tokio::spawn(async move {
        // create the mongo client
        info!("creating mongo client for {}", mongo_connect_string);
        let mongo_client_options =
            mongodb::options::ClientOptions::parse(mongo_connect_string.as_str())
                .await
                .unwrap_or_else(|e| panic!("error creating mongo client options: {:?}", e));
        let mongo_client = mongodb::Client::with_options(mongo_client_options)
            .unwrap_or_else(|e| panic!("error creating mongo client: {:?}", e));

        // start the server
        info!(
            "starting api server on {}:{}",
            server_bind_ip, server_bind_port
        );
        run_api_server(
            server_bind_ip,
            server_bind_port,
            mongo_client,
            shutdown_signal,
        )
        .await;
        info!("api server stopped");
    })
}
