use api::run_server;
use mq::init_mq;
use user_manager::init_usermanager;
use warehouse::store::Warehouse;
use warehouse::package::{MQMessage, EncryptedMessage, SenderInfo, Package, PackageInfo};
use cli::init_cli;
use logging::{init_logging, log_info, log_error};
use pubsub_mq::init_pubsub;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_cli();
    init_mq();
    init_usermanager();
    init_logging().expect("Failed to initialize logging");
    init_pubsub();

    let warehouse = Warehouse::new();

    // Opret en testpakke
    let sender_info = SenderInfo {
        id: "1".into(),
        token: "token123".into(),
        sender: "sender1".into(),
        timestamp: "2024-08-02T12:34:56Z".into(),
    };
    let package = Package { data: "This is a test package".into() };
    let encrypted_message = EncryptedMessage { sender: sender_info, package };
    let mq_message = MQMessage::SendMessage { queue: "test_queue".into(), message: encrypted_message };

    // Gem pakke
    warehouse.store_package(mq_message.clone());

    // Hent og print pakke
    if let Some(fetched_package) = warehouse.fetch_package("test_queue") {
        println!("Fetched package: {:?}", fetched_package);
    }

    // Hent næste pakke i køen
    if let Some(next_package) = warehouse.get_next_package_from_queue("test_queue") {
        println!("Next package in queue: {:?}", next_package);
    }

    run_server().await
}
