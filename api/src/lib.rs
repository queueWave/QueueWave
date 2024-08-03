pub mod package_api;
pub mod storage_api;

pub mod user_api;


use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use warehouse::store::Warehouse;
use user_manager::user::UserManager;
use logging::{log_info};

#[derive(Serialize)]
struct Message {
    content: String,
}

async fn hello() -> impl Responder {
    web::Json(Message {
        content: "Hello from Rust!".to_string(),
    })
}

pub async fn run_server() -> std::io::Result<()> {
    let warehouse = web::Data::new(Warehouse::new());
    let user_manager = web::Data::new(UserManager::new());
    log_info(&format!("Server starts on 0.0.0.0:8080" ));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .app_data(warehouse.clone())
            .app_data(user_manager.clone())
            .service(package_api::add_package)
            .service(package_api::fetch_package)
            .service(package_api::get_pakaget_by_id)
            .service(storage_api::all_packages)
            .service(storage_api::package_size)
            .service(user_api::create_user)
            .service(user_api::login)
            .service(user_api::protected)
            .route("/api/hello", web::get().to(hello))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await

}
