use crate::app::favorite::service::fetch_favorite_info;
use crate::app::tag::model::CreateTag;
use crate::diesel::BelongingToDsl;
use crate::diesel::GroupedBy;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

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
use super::model::CreateArticle;

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

pub struct CreateArticleService {
    pub current_user: User,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_name_list: Option<Vec<String>>,
}

pub fn create_article(
    conn: &mut PgConnection,
    params: &CreateArticleService,
) -> Result<(Article, Profile, FavoriteInfo, Vec<Tag>), AppError> {
    let title_slug = Article::convert_title_to_slug(&params.title);

    let article = Article::create(
        conn,
        &CreateArticle {
            author_id: params.current_user.id,
            slug: title_slug,
            title: params.title.clone(),
            description: params.description.clone(),
            body: params.body.clone(),
        },
    )?;

    let tags_list = create_tags_list(conn, &article.id, &params.tag_name_list)?;
    let profile = params.current_user.get_profile(conn, &article.author_id);
    let favorite_info = fetch_favorite_info(conn, &article.id, &article.author_id)?;
    Ok((article, profile, favorite_info, tags_list))
}

fn create_tags_list(
    conn: &mut PgConnection,
    article_id: &Uuid,
    tag_name_list: &Option<Vec<String>>,
) -> Result<Vec<Tag>, AppError> {
    let list = tag_name_list
        .as_ref()
        .map(|tag_name_list| {
            let records = tag_name_list
                .iter()
                .map(|name| CreateTag { name, article_id })
                .collect();
            Tag::create_tags(conn, records)
        })
        .unwrap_or_else(|| Ok(vec![]));
    list
}
