use serde::{Deserialize, Serialize};

use crate::{
    app::{favorite::model::FavoriteInfo, profile::model::Profile, tag::model::Tag},
    utils::date::Iso8601,
};

use super::model::Article;

#[derive(Deserialize, Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl From<(Article, Profile, FavoriteInfo, Vec<Tag>)> for SingleArticleResponse {
    fn from(
        (article, profile, favorite_info, tags): (Article, Profile, FavoriteInfo, Vec<Tag>),
    ) -> Self {
        Self {
            article: ArticleContent::from((article, profile, &favorite_info, tags)),
        }
    }
}

type ArticleCount = i64;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articles_count: ArticleCount,
}

type ArticlesCount = i64;
type Inner = ((Article, Profile, FavoriteInfo), Vec<Tag>);
type ArticlesList = Vec<Inner>;
type Item = (ArticlesList, ArticlesCount);

impl From<Item> for MultipleArticlesResponse {
    fn from((list, articles_count): (Vec<Inner>, ArticleCount)) -> Self {
        let articles = list
            .iter()
            .map(|((article, profile, favorite_info), tags_list)| {
                ArticleContent::from((
                    article.to_owned(),
                    profile.to_owned(),
                    favorite_info.to_owned(),
                    tags_list.to_owned(),
                ))
            })
            .collect();
        Self {
            articles,
            articles_count,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleContent {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags_list: Vec<String>,
    pub crated_at: Iso8601,
    pub updated_at: Iso8601,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: AuthorCount,
}

impl From<(Article, Profile, &FavoriteInfo, Vec<Tag>)> for ArticleContent {
    fn from(
        (article, profile, favorite_info, tags_list): (Article, Profile, &FavoriteInfo, Vec<Tag>),
    ) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tags_list: tags_list.iter().map(|tag| tag.name.to_string()).collect(),
            crated_at: Iso8601(article.created_at),
            updated_at: Iso8601(article.updated_at),
            favorited: favorite_info.is_favorited.to_owned(),
            favorites_count: favorite_info.favorites_count.to_owned(),
            author: AuthorCount {
                username: profile.username,
                bio: profile.bio,
                image: profile.image,
                following: profile.following,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthorCount {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
