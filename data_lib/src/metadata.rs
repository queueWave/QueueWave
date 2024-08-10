use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub retry_count: u32,
    pub ttl: u32,
    pub tags: Vec<String>,
}
