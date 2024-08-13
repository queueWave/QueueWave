pub mod user;
use logging::{log_info};
use config::get_value;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use crate::user::UserManager;

async fn create_tables(client: &Client) {
    let create_tables_sql = include_str!("../../user_manager/user.sql");
    client.batch_execute(create_tables_sql).await.unwrap();
}


pub async fn init() -> Arc<UserManager> {
    let db_url = get_value("database.url").expect("Database URL not found");
    let username = get_value("database.username").expect("Database username not found");
    let password = get_value("database.password").expect("Database password not found");
    log_info(&format!("Initializing user manager"));
    log_info(&format!("Log on to database with user: {}", username));

    let connection_string = format!("postgres://{}:{}@{}", username, password, db_url);
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await.unwrap();
    let client = Arc::new(client);
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    create_tables(&client).await;

    log_info(&format!("User manager initialized"));

    let user_manager = Arc::new(user::UserManager::new(client));
    if user_manager.login("admin", "admin").await.is_none() {
        user_manager.create_user("admin".to_string(), "admin".to_string()).await;
        log_info(&format!("Created admin user"));
    } else {
        log_info(&format!("Admin user already exists"));
    }

    user_manager
}