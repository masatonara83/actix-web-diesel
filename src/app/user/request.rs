
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupForm {
    pub user: SignupUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SigninFrom {
    pub user: SignInUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignInUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateForm {
    pub user: UpdateUser
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>
}