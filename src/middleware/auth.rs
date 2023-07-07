use std::pin::Pin;

use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::Method,
    web::Data,
    Error, HttpMessage, HttpRequest, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use serde_json::json;
use uuid::Uuid;

use crate::{app::user::model::User, constants, error::AppError, utils::token};

use super::state::AppState;

struct SkipAuthRoute {
    path: &'static str,
    method: Method,
}

impl SkipAuthRoute {
    fn matches_path_and_method(&self, path: &str, method: &Method) -> bool {
        self.matches_path(path) && self.matches_method(method)
    }

    fn matches_path(&self, path: &str) -> bool {
        let expect_path = self.path.split("/").collect::<Vec<_>>();
        let this_path = path.split("/").collect::<Vec<_>>();
        if expect_path.len() != this_path.len() {
            return false;
        }

        let path_set = expect_path.iter().zip(this_path.iter());
        for (expect_path, this_path) in path_set {
            if SkipAuthRoute::is_slug_path(expect_path) {
                continue;
            }
            if expect_path != this_path {
                return false;
            }
        }
        true
    }

    fn matches_method(&self, method: &Method) -> bool {
        self.method == method
    }

    fn is_slug_path(text: &str) -> bool {
        let first = text.chars().next().unwrap_or(' ');
        let last = text.chars().last().unwrap_or(' ');
        first == '{' && last == '}'
    }
}

//Authorization:valueの先頭
const TOKEN_IDENTIFIER: &str = "Token";

//トークン認証を行わないエンドポイント
const SKIP_AUTH_ROUTE: [SkipAuthRoute; 6] = [
    SkipAuthRoute {
        path: "/api/helthcheck",
        method: Method::GET,
    },
    SkipAuthRoute {
        path: "/api/tags",
        method: Method::GET,
    },
    SkipAuthRoute {
        path: "/api/users",
        method: Method::POST,
    },
    SkipAuthRoute {
        path: "/api/users/login",
        method: Method::POST,
    },
    SkipAuthRoute {
        path: "/api/articles",
        method: Method::GET,
    },
    SkipAuthRoute {
        path: "/api/articles/{article_title_slug}/comments",
        method: Method::GET,
    },
];

pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizationMiddleware { service })
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;

    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let is_varified = if should_skip_auth(&req) {
            true
        } else {
            set_auth_user(&mut req)
        };

        if is_varified {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?.map_into_left_body();
                Ok(res)
            })
        } else {
            Box::pin(async move {
                let (req, _res) = req.into_parts();
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                let srv = ServiceResponse::new(req, res);
                Ok(srv)
            })
        }
    }
}

fn should_skip_auth(req: &ServiceRequest) -> bool {
    let method = req.method();
    if Method::OPTIONS == *method {
        return true;
    }

    SKIP_AUTH_ROUTE
        .iter()
        .any(|route| route.matches_path_and_method(req.path(), req.method()))
}

fn set_auth_user(req: &mut ServiceRequest) -> bool {
    match fetch_user(req) {
        Ok(user) => {
            req.extensions_mut().insert(user);
            true
        }
        Err(err_msg) => {
            info!("Cannot fetch user {}", err_msg);
            false
        }
    }
}

fn fetch_user(req: &ServiceRequest) -> Result<User, &str> {
    let user_id = get_user_id_from_header(req)?;

    let conn = &mut req
        .app_data::<Data<AppState>>()
        .ok_or("Cannot get state")
        .and_then(|state| state.conn().map_err(|_| "Cannot get connection"))?;

    User::find(conn, user_id).map_err(|_| "Cannot find auth user")
}

fn get_user_id_from_header(req: &ServiceRequest) -> Result<Uuid, &str> {
    req.headers()
        .get(constants::AUTH_HEADER)
        .ok_or("Cannot find authorization key-value in req header")
        .and_then(|auth_header| auth_header.to_str().map_err(|_| "Cannot stringify"))
        .and_then(|auth_str| {
            if auth_str.starts_with(TOKEN_IDENTIFIER) {
                Ok(auth_str)
            } else {
                Err("Invalid token convention")
            }
        })
        .map(|auth_str| auth_str[TOKEN_IDENTIFIER.len()..auth_str.len()].trim())
        .and_then(|token_str| token::decode(token_str).map_err(|_| "Cannot decede token"))
        .map(|token| token.claims.user_id)
}

pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
    req.extensions()
        .get::<User>()
        .map(|user| user.to_owned())
        .ok_or_else(|| {
            AppError::Unauthorized(json!({
                "error": "Unauthorize user. Need auth token on header"
            }))
        })
}
