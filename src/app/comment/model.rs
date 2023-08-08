use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::{comments, users};

#[derive(Debug, Identifiable, Queryable, Associations, Serialize, Clone)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Comment {
    pub fn find_by_id_and_author_id(
        conn: &mut PgConnection,
        comment_id: &Uuid,
        author_id: &Uuid,
    ) -> Result<Comment, AppError> {
        let comment = comments::table
            .filter(comments::author_id.eq(author_id))
            .find(comment_id)
            .first::<Comment>(conn)?;
        Ok(comment)
    }

    pub fn find_comments_with_author_by_article_id(
        conn: &mut PgConnection,
        article_id: &Uuid,
    ) -> Result<Vec<(Comment, User)>, AppError> {
        let items = comments::table
            .inner_join(users::table)
            .filter(comments::article_id.eq(article_id))
            .get_results::<(Comment, User)>(conn)?;

        Ok(items)
    }
}
