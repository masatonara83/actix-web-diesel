use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::{error::AppError, utils};

type AppConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: utils::db::DbPool,
}

impl AppState {
    pub fn conn(&self) -> Result<AppConn, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}
