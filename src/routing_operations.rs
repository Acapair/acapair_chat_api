use axum::http::StatusCode;

use crate::{
    chat::{Chat, Message},
    AppState,
};

pub async fn receive_message(
    sender: impl ToString,
    data: impl ToString,
    room_id: impl ToString,
    state: &AppState,
) {
    let message = Message::new(sender.to_string(), data.to_string());
    match state.is_chat_exists(&room_id.to_string()).await {
        Some(index) => {
            let mut chats = state.chats.lock().await;
            chats[index].add_message(message, state.max_message_counter);
        }
        None => {
            let mut new_chat = Chat::new(room_id.to_string());
            new_chat.add_message(message, state.max_message_counter);
            let mut chats = state.chats.lock().await;
            let room_id = new_chat.room_id.clone();
            chats.push(new_chat);
            drop(chats);
            tokio::spawn(AppState::chat_destroyer(
                state.chats.clone(),
                room_id,
                state.chat_cleaning_timeout,
            ));
        }
    }
}

pub async fn send_message(
    room_id: impl ToString,
    state: AppState,
) -> (axum::http::StatusCode, std::string::String) {
    match state.is_chat_exists(&room_id.to_string()).await {
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
