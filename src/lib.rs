#![warn(clippy::all)]

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2_redis::RedisConnectionManager;
use std::env;

pub mod models;
pub mod schema;

pub fn db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(&database_url);
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn redis_pool() -> Pool<RedisConnectionManager> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(&*redis_url).unwrap();
    Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", redis_url))
}
