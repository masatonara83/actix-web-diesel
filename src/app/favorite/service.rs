use diesel::PgConnection;
use uuid::Uuid;

use crate::error::AppError;

use super::model::{Favorite, FavoriteInfo};

pub fn fetch_favorite_info(
    conn: &mut PgConnection,
    article_id: &Uuid,
    user_id: &Uuid,
) -> Result<FavoriteInfo, AppError> {
    let is_favorited = Favorite::is_favorited_article_by_user_id(conn, article_id, user_id)?;
    let favorites_count = Favorite::find_favorites_count_by_article_id(conn, article_id)?;
    let favorite_info = FavoriteInfo {
        is_favorited,
        favorites_count,
    };
    Ok(favorite_info)
}
