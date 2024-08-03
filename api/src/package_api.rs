use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use warehouse::store::Warehouse;
use warehouse::package::{MQMessage, EncryptedMessage};

#[derive(Serialize, Deserialize)]
struct PackageInput {
    queue: String,
    message: EncryptedMessage,
}

#[post("api/add_package")]
async fn add_package(data: web::Data<Warehouse>, item: web::Json<PackageInput>) -> impl Responder {
    let mq_message = MQMessage::SendMessage {
        queue: item.queue.clone(),
        message: item.message.clone(),
    };
    data.store_package(mq_message);
    HttpResponse::Ok().json("Package added")
}

#[get("api/fetch_package/{queue}")]
async fn fetch_package(data: web::Data<Warehouse>, queue: web::Path<String>) -> impl Responder {
    let queue = queue.into_inner();
    if let Some(package_info) = data.fetch_package(&queue) {
        HttpResponse::Ok().json(package_info)
    } else {
        HttpResponse::NotFound().json("Package not found")
    }
}
#[get("api/next_package/{queue}")]
async fn next_package(data: web::Data<Warehouse>, queue: web::Path<String>) -> impl Responder {
    let queue = queue.into_inner();
    if let Some(package_info) = data.get_next_package_from_queue(&queue) {
        HttpResponse::Ok().json(package_info)
    } else {
        HttpResponse::NotFound().json("No packages available in queue")
    }
}