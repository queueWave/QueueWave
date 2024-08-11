use tokio::net::TcpStream;
use std::sync::Arc;
use data_lib::storaget::Storaget;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use data_lib::message;
use data_lib::payload::Payload;
use data_lib::metadata::Metadata;
use data_lib::header::Header;





pub async fn handle_session(mut socket: TcpStream, queue_manager: Arc<Storaget>) {
    let mut buffer = [0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => break,
            Ok(n) => {
                let message_str = String::from_utf8_lossy(&buffer[..n]).to_string();
                let received_message: message::Message = match serde_json::from_str(&message_str) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("Failed to parse message: {}", e);
                        if let Err(e) = socket.write_all(format!("Failed to parse message: {}", e).as_bytes()).await {
                            eprintln!("Failed to send response: {}", e);
                        }
                        continue;
                    }
                };
                if (received_message.header.clone().unwrap().x_key.to_string() != "123456789".to_string()) {
                    if let Err(e) = socket.write_all(format!("Message not validet x_key").as_bytes()).await {
                        eprintln!("Failed to send response: {}", e);
                        break;
                    }
                }
                match received_message.command.as_str() {
                    "publish" => {
                        queue_manager.process_message(&received_message.queue_name, serde_json::to_string(&received_message).unwrap()).await;
                        if let Err(e) = socket.write_all(format!("Message stored: {:?}", received_message).as_bytes()).await {
                            eprintln!("Failed to send response: {}", e);
                            break;
                        }
                    }
                    "consume" => {

                        let message = queue_manager.get_message(&received_message.queue_name).await;
                        if let Some(message) = message {
                            if let Err(e) = socket.write_all(message.as_bytes()).await {
                                eprintln!("Failed to send response: {}", e);
                                break;
                            }
                        } else {
                            if let Err(e) = socket.write_all("No message available".as_bytes()).await {
                                eprintln!("Failed to send response: {}", e);
                                break;
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unknown message type: {}", received_message.r#type);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}