use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::{articles, users};

#[derive(Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Article {
    pub fn find_ids_by_author_name(
        conn: &mut PgConnection,
        author_name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        let ids = users::table
            .inner_join(articles::table)
            .filter(users::username.eq(author_name))
            .select(articles::id)
            .load::<Uuid>(conn)?;

        Ok(ids)
    }
}
