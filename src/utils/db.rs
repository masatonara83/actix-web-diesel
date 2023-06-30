use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};
use dotenv::dotenv;

use crate::constants::env_key;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}
