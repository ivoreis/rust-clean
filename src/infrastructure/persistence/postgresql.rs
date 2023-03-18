use diesel::{
    self,
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenv::dotenv;
use std::env;

use crate::config::POSTGRESQL_DB_URI;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool() -> DBConn {
    dotenv().ok();
    let database_url = env::var(POSTGRESQL_DB_URI)
        .expect(&*format!("{value} must be set", value = POSTGRESQL_DB_URI));
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool")
}
