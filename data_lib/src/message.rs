use std::process::Command;
use serde::{Deserialize, Serialize};
use crate::header::Header;
use crate::metadata::Metadata;
use crate::payload::Payload;
use crate::Sender::Sender;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub queue_name: String,
    pub r#type: String,
    pub command: String,
    pub header: Option<Header>,
    pub payload: Option<Payload>,
    pub metadata: Option<Metadata>,
    pub sender: Option<Sender>,

}