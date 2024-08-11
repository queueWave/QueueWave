use std::sync::Arc;
use logging::{log_info};
use data_lib::storaget::Storaget;

mod config;
mod connection;

mod server;
mod session;
mod client;

pub async fn init(storaget: Arc<Storaget>) {
    log_info(&format!("Initializing AMQ library"));
    let config = config::load_config();
    log_info(&format!("AMQ path is localhost:5672"));
    server::start_server(config, storaget).await;
}