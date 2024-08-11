use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use crate::header::Header;
use crate::metadata::Metadata;
use crate::payload::Payload;
use crate::Sender::Sender;

pub struct Storaget {
    pub client: Arc<Client>,
    pub base_path: String,
    pub seen_messages: Mutex<HashMap<String, HashMap<String, bool>>>,
}

impl Storaget {
    pub fn new(client: Arc<Client>, base_path: String) -> Self {
        fs::create_dir_all(&base_path).unwrap();
        Storaget {
            client,
            base_path,
            seen_messages: Mutex::new(HashMap::new()),
        }
    }

    fn get_queue_file_path(&self, queue_name: &str) -> String {
        format!("{}/{}.queue", self.base_path, queue_name)
    }

    pub async fn process_message(&self, queue_name: &str, message: String) {
        let queue_name = queue_name.to_string();
        let message_json: Value = serde_json::from_str(&message).unwrap();


        let header: Header = serde_json::from_value(message_json["header"].clone()).unwrap();
        let payload: Payload = serde_json::from_value(message_json["payload"].clone()).unwrap();
        let metadata: Metadata = serde_json::from_value(message_json["metadata"].clone()).unwrap();
        let sender: Sender = serde_json::from_value(message_json["sender"].clone()).unwrap();

        let header_message_id = header.message_id;
        let header_timestamp = header.timestamp;
        let header_correlation_id = header.correlation_id;
        let metadata_retry_count = metadata.retry_count as i32;
        let metadata_ttl = metadata.ttl as i32;
        let metadata_tags: Vec<String> = metadata.tags;
        let payload_event_type = payload.event_type;
        let payload_data: Value = serde_json::to_value(&payload.data).unwrap();
        let sender_user_name = sender.user.unwrap_or_default();
        let sender_service = sender.service.unwrap_or_default();
        let sender_name = sender.name.unwrap_or_default();
        
        let query = "INSERT INTO combined_message (queue_name, type, header_message_id, header_timestamp, header_correlation_id, metadata_retry_count, metadata_ttl, metadata_tags, payload_event_type, payload_data, sender_user_name, sender_service, sender_name) VALUES ($1, 'type', $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)";
        if let Err(e) = self.client.execute(query, &[
            &queue_name,
            &header_message_id,
            &header_timestamp,
            &header_correlation_id,
            &metadata_retry_count,
            &metadata_ttl,
            &metadata_tags,
            &payload_event_type,
            &payload_data.to_string(),
            &sender_user_name,
            &sender_service,
            &sender_name
        ]).await {
            eprintln!("Error executing query: {:?}", e);
            return;
        }

        // Store message in the file system
        let file_path = self.get_queue_file_path(&queue_name);
        let mut file = OpenOptions::new().append(true).create(true).open(&file_path).unwrap();
        writeln!(file, "{}", message).unwrap();

        println!("Message added to queue '{}': ID {}", queue_name, &header_message_id);
    }

    pub async fn add_message(&self, queue_name: &str, message: String) {
        self.process_message(queue_name, message).await;
    }

    pub async fn get_message(&self, queue_name: &str) -> Option<String> {
        // Retrieve message from the file system
        let file_path = self.get_queue_file_path(queue_name);
        let queue_name = queue_name.to_string();
        if Path::new(&file_path).exists() {
            let file = OpenOptions::new().read(true).write(true).open(&file_path).unwrap();
            let mut reader = BufReader::new(&file);
            let mut lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

            if !lines.is_empty() {
                let message = lines.remove(0);
                fs::write(&file_path, lines.join("\n")).unwrap();

                // Parse the message string into a JSON object
                let message_json: Value = match serde_json::from_str(&message) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("Failed to parse message JSON: {:?}", e);
                        return None;
                    }
                };

                let header_message_id = match message_json["header"]["message_id"].as_str() {
                    Some(id) => id.to_string(),
                    None => {
                        eprintln!("header_message_id not found in message JSON, checking database");

                        // Query the database for the header_message_id
                        let query = "SELECT header_message_id FROM combined_message WHERE queue_name = $1 LIMIT 1";
                        match self.client.query_one(query, &[&queue_name]).await {
                            Ok(row) => row.get(0),
                            Err(e) => {
                                eprintln!("Failed to retrieve header_message_id from database: {:?}", e);
                                return None;
                            }
                        }
                    }
                };

                // Start a new thread to update the database
                let client = self.client.clone();
                tokio::spawn(async move {
                    let update_query = "UPDATE combined_message SET message_status = 'processed' WHERE header_message_id = $1 and queue_name = $2";
                    if let Err(e) = client.execute(update_query, &[&header_message_id, &queue_name]).await {
                        eprintln!("Failed to update message status: {:?}", e);
                    }
                });

                return Some(message);
            }
        }
        None
    }


    pub async fn see_message(&self, queue_name: &str) -> Vec<String> {
        // Retrieve messages from the database
        let query = "SELECT command FROM combined_message WHERE queue_name = $1";
        let rows = self.client.query(query, &[&queue_name]).await.unwrap();
        let mut messages: Vec<String> = rows.iter().map(|row| row.get(0)).collect();

        // Retrieve messages from the file system
        let file_path = self.get_queue_file_path(queue_name);
        if Path::new(&file_path).exists() {
            let file = OpenOptions::new().read(true).open(&file_path).unwrap();
            let reader = BufReader::new(file);
            messages.extend(reader.lines().filter_map(Result::ok));
        }

        messages
    }

    pub async fn list_queues(&self) -> Vec<String> {

        let query = "SELECT DISTINCT queue_name FROM combined_message";
        let rows = self.client.query(query, &[]).await.unwrap();
        let mut queues: Vec<String> = rows.iter().map(|row| row.get(0)).collect();


        let paths = fs::read_dir(&self.base_path).unwrap();
        for entry in paths {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) == Some("queue") {
                if let Some(queue_name) = path.file_stem().and_then(|s| s.to_str()) {
                    queues.push(queue_name.to_string());
                }
            }
        }

        queues
    }

    pub async fn list_pending_messages(&self, queue_name: &str) -> Vec<String> {
        self.see_message(queue_name).await
    }
}