pub mod user;
use logging::{log_info};
pub fn init() {
    log_info(&format!("Initializing user manager"));
    // MQ initialization code here
}
