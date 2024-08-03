use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::sync::{Arc, Mutex};
use logging::{log_info};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UserManager {
    users: Arc<Mutex<Vec<User>>>,
}

impl UserManager {
    pub fn new() -> Self {
        log_info(&format!("Creating UserManager"));
        UserManager { users: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn create_user(&self, id: String, username: String, password: String) -> User {
        log_info(&format!("create User: {}", username));
        let user = User { id, username, password };
        let mut users = self.users.lock().unwrap();
        users.push(user.clone());
        user
    }

    pub fn login(&self, username: &str, password: &str) -> Option<User> {
        log_info(&format!("logging in User: {}", username));
        let users = self.users.lock().unwrap();
        users.iter().find(|user| user.username == username && user.password == password).cloned()
    }
}

impl Serialize for UserManager {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let users = self.users.lock().unwrap();
        users.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserManager {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let users = Vec::deserialize(deserializer)?;
        Ok(UserManager {
            users: Arc::new(Mutex::new(users)),
        })
    }
}
