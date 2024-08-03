use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use user_manager::user::UserManager;

#[derive(Serialize, Deserialize)]
struct UserInput {
    id: String,
    username: String,
    password: String,
}

#[post("api/create_user")]
async fn create_user(data: web::Data<UserManager>, item: web::Json<UserInput>) -> impl Responder {
    let user = data.create_user(item.id.clone(), item.username.clone(), item.password.clone());
    HttpResponse::Ok().json(user)
}

#[derive(Serialize, Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

#[post("api/login")]
async fn login(data: web::Data<UserManager>, item: web::Json<LoginInput>) -> impl Responder {
    if let Some(user) = data.login(&item.username, &item.password) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}
