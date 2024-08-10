use std::sync::Arc;
use user_manager;
use logging;
use data_lib;
use api;
use amq_lib;
use data_lib::storaget;

#[actix_web::main]
async fn main() {
    logging::init();
    user_manager::init();
    let storaget = data_lib::init();
    let storaget_clone = Arc::clone(&storaget);

    let api_future = api::init(storaget);
    let amq_future = amq_lib::init(storaget_clone);

    tokio::join!(api_future, amq_future);
}