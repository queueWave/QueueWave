use std::fmt;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sender {
    pub user: Option<String>,
    pub service: Option<String>,
    pub name: Option<String>,
}


impl fmt::Display for Sender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            self.user.as_deref().unwrap_or("unknown"),
            self.service.as_deref().unwrap_or("unknown"),
            self.name.as_deref().unwrap_or("unknown")
        )
    }
}