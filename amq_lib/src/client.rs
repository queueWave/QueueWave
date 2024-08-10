use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn send_message(host: &str, port: u16, message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
    stream.write_all(message.as_bytes()).await?;
    println!("Sent: {}", message);

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let received_message = String::from_utf8_lossy(&buffer[..n]).to_string();
    println!("Received: {}", received_message);

    Ok(received_message)
}