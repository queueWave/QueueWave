
use config::get_value;
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Config {
    use config::get_value;
    Config {
        host: get_value("amqp.host").expect("AMQ Host not found"),
        port: get_value("amqp.port").expect("AMQ Host not found").parse().unwrap(),
    }
}