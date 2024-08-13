
use std::sync::Arc;
use user_manager;
use logging;
use data_lib;
use api;
use amq_lib;
use data_lib::storaget;

#[actix_web::main]
async fn main() {
    logging::init().expect("Logging failed to initialize");
    let user_manager = user_manager::init().await;
    let user_manager_clone = Arc::clone(&user_manager);
    let storaget = data_lib::init().await;
    let storaget_clone = Arc::clone(&storaget);

    let api_future = api::init(storaget, user_manager_clone);
    let amq_future = amq_lib::init(storaget_clone);

    tokio::join!(api_future, amq_future);
}