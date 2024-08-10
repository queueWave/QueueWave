use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::sync::Arc;
use data_lib::storaget::Storaget;

pub async fn handle_session(mut socket: TcpStream, queue_manager: Arc<Storaget>) {
    let mut buffer = [0; 1024];
    while let Ok(n) = socket.read(&mut buffer).await {
        if n == 0 { break; }

        let message = String::from_utf8_lossy(&buffer[..n]);
        println!("Received message: {}", message);
        // Use QueueManager here if needed for processing messages to queues
        // Example: queue_manager.add_message("some_queue", message.to_string()).await;

        let response = "Message processed\n";
        socket.write_all(response.as_bytes()).await.expect("Failed to send response");
    }
}
