use actix_web::{web, HttpResponse, Responder, Error};
use user_manager::user::{Token, UserManager};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User_request {
    pub username: String,
    pub password: String,
}

async fn create_user(user_manager: web::Data<Arc<UserManager>>, user: web::Json<User_request>) -> impl Responder {
    let new_user = user_manager.create_user(user.username.clone(), user.password.clone()).await;
    HttpResponse::Ok().json(new_user)
}

async fn login(user_manager: web::Data<Arc<UserManager>>, user: web::Json<User_request>) -> Result<impl Responder, Error> {
    println!("Logging in user: {:?}", user);
    if let Some(logged_in_user) = user_manager.login(&user.username, &user.password).await {
        Ok(HttpResponse::Ok().json(logged_in_user))
    } else {
       Ok(HttpResponse::Unauthorized().finish())
    }

}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/user")
            .route("/create", web::post().to(create_user))
            .route("/login", web::post().to(login))
    );
}