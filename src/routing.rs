use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;

use crate::{routing_operations, AppState};

#[derive(Debug, Deserialize)]
struct ReceivedMessage {
    room_id: String,
    username: String,
    message: String,
}

pub async fn routing(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .route("/send", post(receive_message))
        .route("/receive/:room_id", get(send_message))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn alive() -> impl IntoResponse {
    let alive_json = serde_json::json!({
        "server_status":"Alive",
    });
    (StatusCode::OK, Json(alive_json))
}

async fn receive_message(
    State(state): State<AppState>,
    Json(received_message): Json<ReceivedMessage>,
) {
    let sender = received_message.username;
    let data = received_message.message;
    let room_id = received_message.room_id;
    routing_operations::receive_message(sender, data, room_id, &state).await;
}

async fn send_message(
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    routing_operations::send_message(room_id, state).await
}
