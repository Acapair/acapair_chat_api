use std::{net::IpAddr, sync::Arc, time::Duration};

use chat::Chat;
use chrono::Utc;
use tokio::sync::Mutex;

pub mod chat;
pub mod routing;
pub mod routing_operations;
pub mod test;
pub mod utils;

pub struct ServerConfig {
    pub ip_address: IpAddr,
    pub port: u16,
    pub max_message_counter: u16,
    pub chat_cleaning_timeout: u16,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub chats: Arc<Mutex<Vec<Chat>>>,
    pub max_message_counter: u16,
    pub chat_cleaning_timeout: u16,
}

impl AppState {
    pub async fn is_chat_exists(&self, room_id: &String) -> Option<usize> {
        let chats = self.chats.lock().await;
        for i in 0..chats.len() {
            if chats[i].room_id == *room_id {
                return Some(i);
            }
        }
        None
    }

    pub async fn chat_destroyer(
        chats: Arc<Mutex<Vec<Chat>>>,
        room_id: String,
        chat_cleaning_timeout: u16,
    ) {
        let timeout_control = (chat_cleaning_timeout / 10) as u64;
        loop {
            tokio::time::sleep(Duration::from_secs(timeout_control)).await;
            let mut chats = chats.lock().await;
            let current = Utc::now().timestamp() as u64;
            for i in 0..chats.len() {
                if chats[i].room_id == room_id {
                    if chats[i].last_interaction < current - chat_cleaning_timeout as u64 {
                        chats.remove(i);
                        return;
                    }
                }
            }
        }
    }
}
