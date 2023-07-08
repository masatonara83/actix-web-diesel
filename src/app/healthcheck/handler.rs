use actix_web::{HttpResponse, Responder};

pub async fn get_healthcheck() -> impl Responder {
    HttpResponse::Ok().body("OK!")
}
