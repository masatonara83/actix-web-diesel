use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArticleListQueryParameter {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
