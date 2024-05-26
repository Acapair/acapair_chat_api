use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;

use crate::{
    chat::{Chat, Message},
    AppState,
};

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
    State(mut state): State<AppState>,
    Json(received_message): Json<ReceivedMessage>,
) {
    let sender = received_message.username;
    let data = received_message.message;
    let room_id = received_message.room_id;
    let message = Message::new(sender, data);
    match state.is_chat_exists(&room_id).await {
        Some(index) => {
            let mut chats = state.chats.lock().await;
            chats[index].add_message(message, state.max_message_counter);
        }
        None => {
            let mut new_chat = Chat::new(room_id);
            new_chat.add_message(message, state.max_message_counter);
            let mut chats = state.chats.lock().await;
            chats.push(new_chat);
        }
    }
}

async fn send_message(
    Path(room_id): Path<String>, 
    State(mut state): State<AppState>,
) -> impl IntoResponse {
    match state.is_chat_exists(&room_id).await {
        Some(index) => {
            let chats = state.chats.lock().await;
            (
                StatusCode::OK,
                serde_json::to_string(&chats[index]).unwrap(),
            )
        }
        None => (StatusCode::BAD_REQUEST, serde_json::to_string("").unwrap()),
    }
}
