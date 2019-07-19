#![warn(clippy::all)]

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2_redis::RedisConnectionManager;
use sentry::{self, integrations, internals::ClientInitGuard};
use std::env;

pub mod models;
pub mod schema;

pub fn sentry_init() -> ClientInitGuard {
    let guard = sentry::init("https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468");
    env::set_var("RUST_BACKTRACE", "1");
    integrations::env_logger::init(None, Default::default());
    integrations::panic::register_panic_handler();
    guard
}

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
