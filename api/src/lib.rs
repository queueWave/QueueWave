mod storaget_api;
use config::get_value;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use data_lib::storaget::Storaget;
use logging::{log_info};



pub async fn init(storaget: Arc<Storaget>) {
    log_info(&format!("Initializing API library"));
    let host = get_value("api.host").expect("API Host not found");
    let port = get_value("api.port").expect("API Port not found");

    log_info(&format!("API running on {}:{}",host,port));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(storaget.clone()))
            .configure(configure)
    })
        .bind(&format!("{}:{}",host,port))
        .expect(&format!("Cannot bind to port {}:{}",host,port))
        .run()
        .await
        .expect("Failed to run server");

}

pub fn configure(cfg: &mut web::ServiceConfig) {
    storaget_api::configure(cfg);
}