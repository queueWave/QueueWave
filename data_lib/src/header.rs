use serde::{Deserialize, Serialize};
use uuid::Timestamp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub message_id: String,
    pub timestamp: String,
    pub token: String,
    pub x_key: String,
    pub correlation_id: String,
}