// api/src/storaget_api.rs

use actix_web::{web, Responder, HttpResponse};
use std::sync::Arc;
use data_lib::storaget::Storaget;
use logging::{log_info};
use serde_json::Value;

async fn add_message(data: web::Data<Arc<Storaget>>, queue_name: web::Path<String>, message: web::Json<Value>) -> impl Responder {
    data.add_message(&queue_name, message.into_inner().to_string()).await;
    HttpResponse::Ok().body("Message added")
}

async fn get_message(data: web::Data<Arc<Storaget>>, queue_name: web::Path<String>) -> impl Responder {
    if let Some(message) = data.get_message(&queue_name).await {
        if let Ok(json_message) = serde_json::from_str::<Value>(&message) {
            HttpResponse::Ok().json(json_message)
        } else {
            HttpResponse::InternalServerError().body("Failed to parse message as JSON")
        }
    } else {
        HttpResponse::NotFound().body("No message available")
    }
}

async fn see_message(data: web::Data<Arc<Storaget>>, queue_name: web::Path<String>) -> impl Responder {
    let messages = data.see_message(&queue_name).await;
    let json_messages: Vec<Value> = messages.into_iter().filter_map(|msg| serde_json::from_str(&msg).ok()).collect();
    if !json_messages.is_empty() {
        HttpResponse::Ok().json(json_messages)
    } else {
        HttpResponse::NotFound().body("No messages available")
    }
}

async fn list_queues(data: web::Data<Arc<Storaget>>) -> impl Responder {
    let queues = data.list_queues().await;
    HttpResponse::Ok().json(queues)
}

async fn list_pending_messages(data: web::Data<Arc<Storaget>>, queue_name: web::Path<String>) -> impl Responder {
    let messages = data.list_pending_messages(&queue_name).await;
    let json_messages: Vec<Value> = messages.into_iter().filter_map(|msg| serde_json::from_str(&msg).ok()).collect();
    HttpResponse::Ok().json(json_messages)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/queues", web::get().to(list_queues))
            .route("/queues/{queue_name}/add/messages", web::post().to(add_message))
            .route("/queues/{queue_name}/get/messages", web::get().to(get_message))
            .route("/queues/{queue_name}/see/messages", web::get().to(see_message))
            .route("/queues/{queue_name}/pending", web::get().to(list_pending_messages))
    );
}