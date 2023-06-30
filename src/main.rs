use actix_web::{middleware::Logger, App, HttpServer};

extern crate diesel;

#[macro_use]
extern crate log;

mod constants;
mod error;
mod middleware;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("state server ...");
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let state = {
        let pool = utils::db::establish_connection();
        middleware::state::AppState { pool }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .configure(routes::api)
    })
    .bind(constants::BIND_ADDRESS)?
    .run()
    .await
}