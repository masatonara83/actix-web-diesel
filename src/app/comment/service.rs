use diesel::PgConnection;

use crate::{
    app::{article::model::Article, profile::model::Profile},
    error::AppError,
};

use super::model::Comment;

pub struct FetchArticleCommentsService {
    pub slug: String,
}

pub fn fetch_article_comments(
    conn: &mut PgConnection,
    params: &FetchArticleCommentsService,
) -> Result<Vec<(Comment, Profile)>, AppError> {
    let (article, _author) = Article::find_by_slug_with_author(conn, &params.slug)?;
    let list = Comment::find_comments_with_author_by_article_id(conn, &article.id)?;

    let comments_with_authors = list
        .iter()
        .map(|(comment, user)| {
            let profile = user.get_profile(conn, &user.id);

            (comment.to_owned(), profile)
        })
        .collect::<Vec<(Comment, Profile)>>();

    Ok(comments_with_authors)
}
