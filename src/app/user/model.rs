use crate::error::AppError;
use crate::schema::users;
use crate::utils::{hasher, token};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Token = String;

//User構造体
#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
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
        let password_hash = hasher::hash_password(password)?;
        let recode = SignupUser {
            username,
            email,
            password: &password_hash,
        };

        let user = diesel::insert_into(users::table)
            .values(&recode)
            .get_result::<User>(conn)?;

        let token = user.gerenate_token()?;
        Ok((user, token))
    }

    pub fn authenticate(
        conn: &mut PgConnection,
        email: &str,
        password: &str,
    ) -> Result<(User, Token), AppError> {
        let user = users::table
            .filter(users::email.eq(email))
            .limit(1)
            .first::<User>(conn)?;

        hasher::verify(password, &user.password)?;
        let token = user.gerenate_token()?;
        Ok((user, token))
    }

    pub fn gerenate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp_nanos();
        let token = token::encode(self.id, now)?;
        Ok(token)
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
struct SignupUser<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}
