pub mod storaget;
pub mod message;
pub mod payload;
pub mod metadata;
pub mod header;
mod Sender;
use config::get_value;
use std::sync::Arc;
use logging::{log_info};
use tokio_postgres::{Client, NoTls};

async fn create_tables(client: &Client) {
    let create_tables_sql = include_str!("../../data_lib/storaget.sql");
    client.batch_execute(create_tables_sql).await.unwrap();
}

pub async fn init() -> Arc<storaget::Storaget> {
    let db_url = get_value("database.url").expect("Database URL not found");
    let username = get_value("database.username").expect("Database username not found");
    let password = get_value("database.password").expect("Database password not found");
    log_info(&format!("Initializing Storaget for user: {}", username));
    log_info(&format!("File Storaget is created in tmp"));
    log_info(&format!("Log on to database with user: {}", username));


    let connection_string = format!("postgres://{}:{}@{}", username, password, db_url);
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    create_tables(&client).await;
    let base_path = "tmp";

    Arc::new(storaget::Storaget::new(Arc::new(client), base_path.to_string()))
}