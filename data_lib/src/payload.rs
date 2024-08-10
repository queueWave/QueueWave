use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub event_type: String,
    pub data: std::collections::HashMap<String, String>,
}