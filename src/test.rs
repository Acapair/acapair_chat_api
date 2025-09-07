#[cfg(test)]
use crate::{routing_operations::receive_message, AppState};
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use std::time::Duration;
#[cfg(test)]
use tokio::sync::Mutex;

#[tokio::test]
async fn new_message_no_chat() {
    let state = AppState {
        chats: Arc::new(Mutex::new(vec![])),
        max_message_counter: 5,
        chat_cleaning_timeout: 10,
    };
    receive_message("Tahinli", "Hi", "Tahinli-Chat", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].messages.len(), 1);
}

#[tokio::test]
async fn new_message_with_chat() {
    let state = AppState {
        chats: Arc::new(Mutex::new(vec![])),
        max_message_counter: 5,
        chat_cleaning_timeout: 10,
    };
    receive_message("Tahinli", "Hi", "Tahinli-Chat", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].messages.len(), 1);
    drop(chats);

    receive_message("Tahinli", "Hi Again", "Tahinli-Chat", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].messages.len(), 2);
}

#[tokio::test]
async fn chat_auto_deletion_with_timeout() {
    let state = AppState {
        chats: Arc::new(Mutex::new(vec![])),
        max_message_counter: 5,
        chat_cleaning_timeout: 1,
    };
    receive_message("Tahinli", "Hi", "Tahinli-Chat", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].messages.len(), 1);
    drop(chats);

    tokio::time::sleep(Duration::from_secs(2)).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 0);
}

#[tokio::test]
async fn message_auto_deletion_with_counter() {
    let state = AppState {
        chats: Arc::new(Mutex::new(vec![])),
        max_message_counter: 1,
        chat_cleaning_timeout: 10,
    };
    receive_message("Tahinli", "Hi", "Tahinli-Chat", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].messages.len(), 1);
    drop(chats);

    receive_message("Tahinli", "Hi", "Tahinli-Chat", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].messages.len(), 1);
}

#[tokio::test]
async fn create_multiple_chats() {
    let state = AppState {
        chats: Arc::new(Mutex::new(vec![])),
        max_message_counter: 1,
        chat_cleaning_timeout: 10,
    };
    receive_message("Tahinli", "Hi", "Tahinli-Chat-1", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].messages.len(), 1);
    drop(chats);

    receive_message("Tahinli", "Hi", "Tahinli-Chat-2", &state).await;
    let chats = state.chats.lock().await;
    assert_eq!(chats.len(), 2);
    assert_eq!(chats[1].messages.len(), 1);
}
