use actix_web::{post, web, HttpResponse, Responder, HttpServer, App, HttpRequest};
use serde::{Deserialize, Serialize};
use user_manager::user::UserManager;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use std::time::{SystemTime, UNIX_EPOCH};
use actix_http::header::AUTHORIZATION;

const SECRET_KEY: &[u8] = b"kdsjfklsdjfkldsjklfjdskfjksdjfslk";

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct UserInput {
    username: String,
    password: String,
}

#[post("api/create_user")]
async fn create_user(data: web::Data<UserManager>, item: web::Json<UserInput>) -> impl Responder {
    println!("create user called ");
    let user = data.create_user(item.username.clone(), item.password.clone());
    HttpResponse::Ok().json(user)
}

#[derive(Serialize, Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[post("api/login")]
async fn login(data: web::Data<UserManager>, item: web::Json<LoginInput>) -> impl Responder {
    if let Some(user) = data.login(&item.username, &item.password) {
        let my_claims = Claims {
            sub: user.id.clone(),
            exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 60 * 60) as usize, // 1 hour expiration
        };
        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(SECRET_KEY)).unwrap();
        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}

async fn validate_token(token: &str) -> Result<Claims, HttpResponse> {
    match decode::<Claims>(&token, &DecodingKey::from_secret(SECRET_KEY), &Validation::new(Algorithm::HS256)) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(HttpResponse::Unauthorized().json("Invalid token")),
    }
}

async fn protected_route(data: web::Data<UserManager>, token: &str) -> HttpResponse {
    match validate_token(token).await {
        Ok(claims) => {
            // Access to protected data
            HttpResponse::Ok().json(format!("Welcome user with ID: {}", claims.sub))
        }
        Err(e) => e,
    }
}

#[post("api/protected")]
async fn protected(data: web::Data<UserManager>, req: HttpRequest) -> HttpResponse {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                return protected_route(data, token).await;
            }
        }
    }
    HttpResponse::Unauthorized().json("Missing or malformed token")
}
