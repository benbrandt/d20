#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]
use async_std::io;
use d20::{r2d2_rng::RngConnectionManager, redis_pool, rng_pool, sentry_init};
use diesel::r2d2::Pool;
use dotenv::dotenv;
use r2d2_redis::RedisConnectionManager;
use std::env;
use tide::{security::CorsMiddleware, Server};

mod handlers;

// First, we define `State` that holds accumulator state. This is accessible as state in
// Tide, and as executor context in Juniper.
#[derive(Clone)]
pub struct State {
    redis: Pool<RedisConnectionManager>,
    rng: Pool<RngConnectionManager>,
}

impl Default for State {
    #[must_use]
    fn default() -> Self {
        Self {
            redis: redis_pool(),
            rng: rng_pool(),
        }
    }
}

#[async_std::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let _guard = sentry_init();

    // Get the port number to listen on.
    let port: i32 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    let mut app = Server::with_state(State::default());

    app.with(CorsMiddleware::new());
    //     .with(Compression::new())
    //     .with(Decompression::new());

    app.at("/roll/")
        .get(handlers::parse_roll)
        .post(handlers::roll);

    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
