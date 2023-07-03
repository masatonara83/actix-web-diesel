use actix_web::web::{post, scope, ServiceConfig};

use crate::app::user::handlers::{signin, signup};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .route("/users", post().to(signup))
            .route("/users/login", post().to(signin)),
    );
}
