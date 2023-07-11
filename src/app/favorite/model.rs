use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::{favorites, users};

pub struct FavoriteInfo {
    pub is_favorited: bool,
    pub favorites_count: i64,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Favorite {
    pub fn find_favorited_article_ids_by_username(
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        let article_ids = favorites::table
            .inner_join(users::table)
            .filter(users::username.eq(username))
            .select(favorites::article_id)
            .load::<Uuid>(conn)?;
        Ok(article_ids)
    }

    pub fn find_favorites_count_by_article_id(
        conn: &mut PgConnection,
        article_id: &Uuid,
    ) -> Result<i64, AppError> {
        let count = favorites::table
            .filter(favorites::article_id.eq(article_id))
            .select(diesel::dsl::count(favorites::id))
            .first::<i64>(conn)?;
        Ok(count)
    }

    pub fn is_favorited_article_by_user_id(
        conn: &mut PgConnection,
        article_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<bool, AppError> {
        let count = favorites::table
            .filter(favorites::article_id.eq(article_id))
            .filter(favorites::user_id.eq(user_id))
            .select(diesel::dsl::count(favorites::id))
            .first::<i64>(conn)?;
        let is_favorited = count > 0;
        Ok(is_favorited)
    }
}
