use crate::{error::AppError, schema::users};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Token = String;

//User構造体
#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    //Create new User
    pub fn create<'a>(
        conn: &mut PgConnection,
        username: &'a str,
        email: &'a str,
        password: &'a str,
    ) -> Result<(User, Token), AppError> {
    }
}
