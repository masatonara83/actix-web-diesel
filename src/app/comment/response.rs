use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::profile::model::Profile, utils::date::Iso8601};

use super::model::Comment;

#[derive(Deserialize, Serialize)]
pub struct MultipleCommentsResponse {
    pub comments: Vec<InnerComment>,
}

impl From<Vec<(Comment, Profile)>> for MultipleCommentsResponse {
    fn from(list: Vec<(Comment, Profile)>) -> Self {
        Self {
            comments: list
                .into_iter()
                .map(|item| {
                    let (comment, profile) = item;
                    InnerComment::from((&comment, &profile))
                })
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerComment {
    pub id: Uuid,
    pub author: InnerAuthor,
    pub body: String,
    pub created_at: Iso8601,
    pub updated_at: Iso8601,
}

impl From<(&Comment, &Profile)> for InnerComment {
    fn from((comment, profile): (&Comment, &Profile)) -> Self {
        Self {
            id: comment.id,
            author: InnerAuthor::from(profile),
            body: comment.body.to_string(),
            created_at: Iso8601(comment.created_at),
            updated_at: Iso8601(comment.updated_at),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerAuthor {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<&Profile> for InnerAuthor {
    fn from(profile: &Profile) -> Self {
        Self {
            username: profile.username.to_owned(),
            bio: profile.bio.to_owned(),
            image: profile.image.to_owned(),
            following: profile.following.to_owned(),
        }
    }
}
