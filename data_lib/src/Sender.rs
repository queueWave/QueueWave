use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sender {
    pub user: Option<String>,
    pub service: Option<String>,
    pub name: Option<String>,
}