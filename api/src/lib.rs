mod storaget_api;
mod user_api;
mod token_api;

use config::get_value;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use actix_cors::Cors;
use data_lib::storaget::Storaget;
use logging::{log_info};
use user_manager::user::UserManager;

pub async fn init(storaget: Arc<Storaget>, user_manager: Arc<UserManager>) {
    log_info(&"Initializing API library".to_string());
    let host = get_value("api.host").expect("API Host not found");
    let port = get_value("api.port").expect("API Port not found");

    log_info(&format!("API running on {}:{}",host,port));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .app_data(web::Data::new(storaget.clone()))
            .app_data(web::Data::new(user_manager.clone()))
            .configure(configure)
    })
        .bind(&format!("{}:{}",host,port))
        .expect(&format!("Cannot bind to port {}:{}",host,port))
        .run()
        .await
        .expect("Failed to run server");

}

pub fn configure(cfg: &mut web::ServiceConfig) {
    user_api::configure(cfg);
    storaget_api::configure(cfg);
    token_api::configure(cfg);
}