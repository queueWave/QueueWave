use actix_web::{web, HttpResponse, Responder};
use user_manager::user::{UserManager, Token};
use std::sync::Arc;

async fn create_token(user_manager: web::Data<Arc<UserManager>>, user_id: web::Path<i32>) -> impl Responder {
    if let Some(token) = user_manager.create_token(user_id.into_inner()).await {
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn get_tokens(user_manager: web::Data<Arc<UserManager>>, user_id: web::Path<i32>) -> impl Responder {
    let tokens = user_manager.get_tokens(user_id.into_inner()).await;
    HttpResponse::Ok().json(tokens)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/token")
            .route("/create/{user_id}", web::post().to(create_token))
            .route("/get/{user_id}", web::get().to(get_tokens))
    );
}