#[macro_use]
extern crate log;

pub mod pubsub;

use logging::log_info;

pub fn init_pubsub() {
    log_info("Initializing PubSub Message Queue");
    // Additional initialization code here if needed
}
