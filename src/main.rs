use std::{net::SocketAddr, sync::Arc};

use acapair_chat_api::{
    routing::routing,
    utils::{read_server_config, tls_config},
    AppState,
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let server_config = read_server_config();

    let tls_config = tls_config().await;

    let state = AppState {
        chats: Arc::new(Mutex::new(vec![])),
        max_message_counter: server_config.max_message_counter,
        chat_cleaning_timeout: server_config.chat_cleaning_timeout,
    };
    let app = routing(axum::extract::State(state)).await;
    let addr = SocketAddr::new(server_config.ip_address, server_config.port);
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
