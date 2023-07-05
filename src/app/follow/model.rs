use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::user::model::User;
use crate::schema::follows;

#[derive(Queryable, Associations, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = follower_id, foreign_key = followee_id))]
#[diesel(table_name = follows)]
pub struct Follow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Follow {
    //Check if a user is Following another user
    pub fn is_following(conn: &mut PgConnection, follower_id: &Uuid, followee_id: &Uuid) -> bool {
        let follow = follows::table
            .filter(follows::follower_id.eq(follower_id))
            .filter(follows::followee_id.eq(followee_id))
            .get_result::<Follow>(conn);
        follow.is_ok()
    }
}
