use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::article::model::Article;
use crate::error::AppError;
use crate::schema::tags;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, Associations)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub article_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Tag {
    pub fn find_ids_by_name(
        conn: &mut PgConnection,
        tag_name: &str,
    ) -> Result<Vec<Uuid>, AppError> {
        let tag_ids = tags::table
            .filter(tags::name.eq(tag_name))
            .select(tags::id)
            .get_results::<Uuid>(conn)?;
        Ok(tag_ids)
    }
}
