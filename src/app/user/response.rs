use super::model::User;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserResponse {
    pub user: AuntUser,
}

impl From<(User, String)> for UserResponse {
    fn from((user, token): (User, String)) -> Self {
        Self {
            user: AuntUser {
                username: user.username,
                email: user.email,
                bio: user.bio,
                image: user.image,
                token,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuntUser {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
