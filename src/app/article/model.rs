use std::convert;

use chrono::NaiveDateTime;
use convert_case::Converter;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::{articles, users};
use crate::utils::converter;

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
    pub fn convert_title_to_slug(title: &str) -> String {
        converter::to_kebab(title)
    }

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

    pub fn create(conn: &mut PgConnection, recode: &CreateArticle) -> Result<Self, AppError> {
        let article = diesel::insert_into(articles::table)
            .values(recode)
            .get_result::<Article>(conn)?;

        Ok(article)
    }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = articles)]
pub struct CreateArticle {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}
