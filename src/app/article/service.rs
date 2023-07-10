use crate::diesel::BelongingToDsl;
use crate::diesel::GroupedBy;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
    app::{
        favorite::model::{Favorite, FavoriteInfo},
        profile::model::Profile,
        tag::model::Tag,
        user::model::User,
    },
    error::AppError,
    schema::{articles, tags, users},
};

use super::model::Article;

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

type ArticlesCount = i64;
type ArticlesListInner = (Article, Profile, FavoriteInfo);
type ArticlesList = Vec<(ArticlesListInner, Vec<Tag>)>;

pub fn fetch_articles_list(
    conn: &mut PgConnection,
    params: FetchArticlesList,
) -> Result<(ArticlesList, ArticlesCount), AppError> {
    let query = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let ids = Tag::find_ids_by_name(conn, tag_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(author_name) = &params.author {
            let ids = Article::find_ids_by_author_name(conn, author_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(username) = &params.favorited {
            let ids = Favorite::find_favorited_article_ids_by_username(conn, username)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        query
    };

    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)?;

    let query = {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let ids = Tag::find_ids_by_name(conn, tag_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(author_name) = &params.author {
            let ids = Article::find_ids_by_author_name(conn, author_name)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        if let Some(username) = &params.favorited {
            let ids = Favorite::find_favorited_article_ids_by_username(conn, username)?;
            query = query.filter(articles::id.eq_any(ids));
        }

        query
    };

    let article_and_user_list = query
        .offset(params.offset)
        .limit(params.limit)
        .load::<(Article, User)>(conn)?;

    let tag_list = {
        let articles_list = article_and_user_list
            .clone()
            .into_iter()
            .map(|(article, _)| article)
            .collect::<Vec<_>>();

        let tags_list = Tag::belonging_to(&articles_list)
            .order(tags::name.asc())
            .load::<Tag>(conn)?;

        let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);

        tags_list
    };

    let favorite_count_list = {
        let list: Result<Vec<i64>, _> = article_and_user_list
            .clone()
            .into_iter()
            .map(|(article, _)| Favorite::find_favorites_count_by_article_id(conn, &article.id))
            .collect();

        list?
    };

    let articles_list = article_and_user_list
        .into_iter()
        .zip(favorite_count_list)
        .map(|((article, user), favorites_count)| {
            (
                article,
                Profile {
                    username: user.username,
                    bio: user.bio,
                    image: user.image,
                    following: false,
                },
                FavoriteInfo {
                    is_favorited: false,
                    favorites_count,
                },
            )
        })
        .zip(tag_list)
        .collect::<Vec<_>>();

    Ok((articles_list, articles_count))
}
