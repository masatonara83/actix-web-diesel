use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

use crate::app::follow::handlers::{create_follow, delete_follow};
use crate::app::profile::handlers::get_profile;
use crate::app::user::handlers::{get_user, signin, signup, update_user};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            // .route("helthcheck", get().to(get_helthcheck))
            .route("/users", post().to(signup))
            .route("/users/login", post().to(signin))
            .route("/user", get().to(get_user))
            .route("/user", put().to(update_user))
            .route("/profiles/{username}", get().to(get_profile))
            .route("/profiles/{username}/follow", post().to(create_follow))
            .route("/profiles/{username}/follow", delete().to(delete_follow)),
    );
}
