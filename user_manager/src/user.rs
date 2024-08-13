use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use chrono::{Utc, Duration};
use logging::log_info;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub tokens: Vec<Token>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub access_token: String,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
pub struct UserManager {
    db_client: Arc<Client>,
}

impl UserManager {
    pub fn new(db_client: Arc<Client>) -> Self {
        log_info(&"Creating UserManager".to_string());
        UserManager { db_client }
    }

    pub async fn create_user(&self, username: String, password: String) -> User {
        log_info(&format!("create User: {}", username));
        let row = self.db_client.query_one(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
            &[&username, &password],
        ).await.unwrap();
        let id: i32 = row.get(0);
        User {
            id,
            username,
            password,
            tokens: Vec::new(),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Option<User> {
        log_info(&format!("logging in User: {}", username));
        let row = self.db_client.query_opt(
            "SELECT id, username, password FROM users WHERE username = $1 AND password = $2",
            &[&username, &password],
        ).await.unwrap();

        if let Some(row) = row {
            Some(User {
                id: row.get(0),
                username: row.get(1),
                password: row.get(2),
                tokens: Vec::new(),
            })
        } else {
            None
        }
    }

    pub async fn create_token(&self, user_id: i32) -> Option<Token> {
        let token = Token {
            id: 0,
            user_id,
            access_token: Uuid::new_v4().to_string(),
            expires_at: (Utc::now() + Duration::days(30)).to_rfc3339(),
        };
        let row = self.db_client.query_one(
            "INSERT INTO tokens (user_id, access_token, expires_at) VALUES ($1, $2, $3) RETURNING id",
            &[&token.user_id, &token.access_token, &token.expires_at],
        ).await.unwrap();
        let id: i32 = row.get(0);
        Some(Token { id, ..token })
    }

    pub async fn get_tokens(&self, user_id: i32) -> Vec<Token> {
        let rows = self.db_client.query(
            "SELECT id, user_id, access_token, expires_at FROM tokens WHERE user_id = $1",
            &[&user_id],
        ).await.unwrap();

        rows.iter().map(|row| Token {
            id: row.get(0),
            user_id: row.get(1),
            access_token: row.get(2),
            expires_at: row.get(3),
        }).collect()
    }
}