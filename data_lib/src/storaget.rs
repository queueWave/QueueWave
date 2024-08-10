use std::collections::{HashMap, VecDeque};
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct Storaget {
    pub queues: Mutex<HashMap<String, VecDeque<String>>>,
}

impl Storaget {
    pub fn new() -> Self {
        Storaget {
            queues: Mutex::new(HashMap::new()),
        }
    }

    pub async fn process_message(&self, queue_name: &str, message: String) {
        let mut queues = self.queues.lock().await;
        let queue = queues.entry(queue_name.to_string()).or_insert_with(VecDeque::new);
        queue.push_back(message.clone());
        println!("Message added to queue '{}': {}", queue_name, message);
    }

    pub async fn add_message(&self, queue_name: &str, message: String) {
        self.process_message(queue_name, message).await;
    }

    pub async fn get_message(&self, queue_name: &str) -> Option<String> {
        let mut queues = self.queues.lock().await;
        if let Some(queue) = queues.get_mut(queue_name) {
            let message = queue.pop_front();
            message
        } else {
            None
        }
    }
    pub async fn see_message(&self, queue_name: &str) -> Vec<String> {
        let queues = self.queues.lock().await;
        if let Some(queue) = queues.get(queue_name) {
            queue.iter().cloned().collect()
        } else {
            vec![]
        }
    }

    pub async fn list_queues(&self) -> Vec<String> {
        let queues = self.queues.lock().await;
        queues.keys().cloned().collect()
    }

    pub async fn list_pending_messages(&self, queue_name: &str) -> Vec<String> {
        let queues = self.queues.lock().await;
        if let Some(queue) = queues.get(queue_name) {
            queue.iter().cloned().collect()
        } else {
            vec![]
        }
    }
}
