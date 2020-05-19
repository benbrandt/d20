#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2_redis::RedisConnectionManager;
use r2d2_rng::RngConnectionManager;
use sentry::{self, integrations, internals::ClientInitGuard};
use std::env;

pub mod dice_roller;
pub mod models;
pub mod r2d2_rng;
pub mod schema;

pub const REDIS_KEY_ROLL_STATS: &str = "roll_stats";

pub fn sentry_init() -> ClientInitGuard {
    let guard = sentry::init("https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468");
    env::set_var("RUST_BACKTRACE", "1");
    tide::log::start();
    integrations::panic::register_panic_handler();
    guard
}

#[must_use]
pub fn db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(&database_url);
    Pool::builder()
        .max_size(9)
        .min_idle(Some(1))
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[must_use]
pub fn redis_pool() -> Pool<RedisConnectionManager> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(&*redis_url).unwrap();
    Pool::builder()
        .max_size(9)
        .min_idle(Some(1))
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", redis_url))
}

#[must_use]
pub fn rng_pool() -> Pool<RngConnectionManager> {
    let manager = RngConnectionManager::new();
    Pool::builder()
        .min_idle(Some(1))
        .build(manager)
        .unwrap_or_else(|_| panic!("Error creating rngs"))
}
