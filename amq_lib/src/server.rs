use tokio::net::TcpListener;
use std::sync::Arc;
use crate::session::handle_session;
use crate::Storaget;

pub async fn start_server(config: crate::config::Config, storaget: Arc<Storaget>) {
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await.unwrap();
    println!("Server running on {}:{}", config.host, config.port);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New connection: {}", addr);
        let queue_manager_clone = storaget.clone();
        tokio::spawn(async move {
            handle_session(socket, queue_manager_clone).await;
        });
    }
}