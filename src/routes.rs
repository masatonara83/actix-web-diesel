use actix_web::web::{get, post, scope, ServiceConfig};

use crate::app::user::handlers::signup;

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api").route("/users", post().to(signup)), // .route("/user", get().to(get_user)),
    );
}
