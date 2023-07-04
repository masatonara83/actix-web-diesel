use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::user::model::User;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl User {
    pub fn get_profile(&self, conn: &mut PgConnection, followee_id: &Uuid) -> Profile {
        let is_following = Follow::is_following(conn, &self.id, followee_id);

        Profile {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following: is_following,
        }
    }
}
