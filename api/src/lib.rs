mod storaget_api;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use data_lib::storaget::Storaget;
use logging::{log_info};



pub async fn init(storaget: Arc<Storaget>) {
    log_info(&format!("Initializing API library"));
    log_info(&format!("running on 127.0.0.1:5000"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(storaget.clone()))
            .configure(configure)
    })
        .bind("127.0.0.1:5000")
        .expect("Cannot bind to port 8080")
        .run()
        .await
        .expect("Failed to run server");

}

pub fn configure(cfg: &mut web::ServiceConfig) {
    storaget_api::configure(cfg);
}