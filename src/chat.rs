use std::collections::VecDeque;

use chrono::Utc;
use serde::Serialize;
use sha3::{Digest, Sha3_512};

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    hash: String,
    sender: String,
    time_received: u64,
    data: String,
}
#[derive(Debug, Serialize)]
pub struct Chat {
    pub room_id: String,
    pub messages: VecDeque<Message>,
    pub last_interaction: u64,
}

impl Chat {
    pub fn new(room_id: String) -> Self {
        Chat {
            room_id,
            messages: vec![].into(),
            last_interaction: Utc::now().timestamp() as u64,
        }
    }

    pub fn add_message(&mut self, mut message: Message, max_message_count: u16) -> bool {
        message.calculate_hash();
        if !self.is_message_exists(message.clone()) {
            self.messages.push_back(message);
            self.last_interaction = Utc::now().timestamp() as u64;
            if self.messages.len() > max_message_count as usize {
                let _ = self.messages.pop_front();
            }
            return true;
        }
        false
    }

    fn is_message_exists(&self, new_message: Message) -> bool {
        for message in &self.messages {
            if new_message.hash == message.hash {
                return true;
            }
        }
        false
    }
}

impl Message {
    pub fn new(sender: String, data: String) -> Self {
        Message {
            hash: String::new(),
            sender,
            time_received: Utc::now().timestamp_millis() as u64,
            data,
        }
    }
    fn calculate_hash(&mut self) {
        let message = Message {
            hash: String::new(),
            sender: self.sender.clone(),
            time_received: self.time_received,
            data: self.data.clone(),
        };
        let message = serde_json::to_string(&message).unwrap();

        let mut hasher = Sha3_512::new();
        hasher.update(message);
        let hash = hasher.finalize();
        self.hash = format!("{:x}", hash);
    }
}
