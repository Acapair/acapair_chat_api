use std::{net::IpAddr, sync::Arc};

use chat::Chat;
use tokio::sync::Mutex;

pub mod chat;
pub mod routing;
pub mod test;
pub mod utils;

pub struct ServerConfig {
    pub ip_address: IpAddr,
    pub port: u16,
    pub max_message_counter: u64,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub chats: Arc<Mutex<Vec<Chat>>>,
    pub max_message_counter: u64,
}

impl AppState {
    pub async fn is_chat_exists(&mut self, room_id: &String) -> Option<usize> {
        let chats = self.chats.lock().await;
        for i in 0..chats.len() {
            if chats[i].room_id == *room_id {
                return Some(i);
            }
        }
        None
    }
}
