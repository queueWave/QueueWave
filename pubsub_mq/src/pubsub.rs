use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub topic: String,
    pub content: String,
}

pub struct PubSub {
    subscribers: HashMap<String, mpsc::Sender<Message>>,
}

impl PubSub {
    pub fn new() -> Self {
        PubSub {
            subscribers: HashMap::new(),
        }
    }

    pub async fn publish(&mut self, message: Message) {
        let subscribers = self.subscribers.clone();
        for (_id, tx) in subscribers {
            let _ = tx.send(message.clone()).await;
        }
    }

    pub async fn subscribe(&mut self, client_id: String) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(100);
        self.subscribers.insert(client_id, tx);
        rx
    }
}
