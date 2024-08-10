pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Config {
    Config {
        host: "127.0.0.1".to_string(),
        port: 5672,
    }
}