use actix_web::web::{get, post, put, scope, ServiceConfig};

use crate::app::user::handlers::{get_user, signin, signup, update_user};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            // .route("helthcheck", get().to(get_helthcheck))
            .route("/users", post().to(signup))
            .route("/users/login", post().to(signin))
            .route("/user", get().to(get_user))
            .route("/user", put().to(update_user))
            .route("/profiles/{username}", get().to(get_profile)),
    );
}
