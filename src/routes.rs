use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

use crate::app::article::handlers::{
    create_article, delete_article, get_article_by_slug, get_articles, get_articles_feed,
    update_article,
};
use crate::app::follow::handlers::{create_follow, delete_follow};
use crate::app::healthcheck::handler::get_healthcheck;
use crate::app::profile::handlers::get_profile;
use crate::app::user::handlers::{get_user, signin, signup, update_user};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .route("/healthcheck", get().to(get_healthcheck))
            .route("/users", post().to(signup))
            .route("/users/login", post().to(signin))
            .route("/user", get().to(get_user))
            .route("/user", put().to(update_user))
            .route("/profiles/{username}", get().to(get_profile))
            .route("/profiles/{username}/follow", post().to(create_follow))
            .route("/profiles/{username}/follow", delete().to(delete_follow))
            .route("/articles", get().to(get_articles))
            .route("/articles", post().to(create_article))
            .route("/articles/feed", get().to(get_articles_feed))
            .route("/articles/{slug}", get().to(get_article_by_slug))
            .route("/articles/{slug}", put().to(update_article))
            .route("/articles/{slug}", delete().to(delete_article)),
    );
}
