
pub mod storaget;
pub mod message;
pub mod payload;
pub mod metadata;
pub mod header;
mod Sender;

use std::sync::Arc;
use logging::{log_info};


// data_lib/src/lib.rs
pub fn init() -> Arc<storaget::Storaget> {
    log_info(&format!("Initializing data library"));
    Arc::new(storaget::Storaget::new())
}