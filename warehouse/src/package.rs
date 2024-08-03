use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SenderInfo {
    pub id: String,
    pub token: String,
    pub sender: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedMessage {
    pub sender: SenderInfo,
    pub package: Package,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MQMessage {
    SendMessage { queue: String, message: EncryptedMessage },
    ReceiveMessage { queue: String, token: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageInfo {
    pub id: String,
    pub received: String,
    pub size: usize,
    pub message: MQMessage,
}
