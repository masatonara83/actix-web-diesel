use std::env;

use actix_cors::Cors;
use actix_web::http;

use crate::constants::env_key;

pub fn cors() -> Cors {
    let frontend_origin = env::var(env_key::FRONTEND_ORIGIN).expect("FRONTEND_ORIGIN must be set");

    Cors::default()
        .allowed_origin(&frontend_origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
