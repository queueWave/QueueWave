use actix_web::{get, web, HttpResponse, Responder};
use warehouse::store::Warehouse;

#[get("api/all_packages")]
async fn all_packages(data: web::Data<Warehouse>) -> impl Responder {
    let packages = data.get_all_packages();
    HttpResponse::Ok().json(packages)
}

#[get("api/package_size/{sender}")]
async fn package_size(data: web::Data<Warehouse>, sender: web::Path<String>) -> impl Responder {
    let sender = sender.into_inner();
    let size = data.get_package_size(&sender);
    HttpResponse::Ok().json(size)
}
