#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
use d20::{r2d2_rng::RngConnectionManager, redis_pool, rng_pool, sentry_init};
use diesel::r2d2::Pool;
use dotenv::dotenv;
use futures::executor::block_on;
use r2d2_redis::RedisConnectionManager;
use std::env;
use tide::{middleware::RequestLogger, Server};

mod dice_roller;
mod handlers;

// First, we define `State` that holds accumulator state. This is accessible as state in
// Tide, and as executor context in Juniper.
#[derive(Clone)]
pub struct State {
    redis: Pool<RedisConnectionManager>,
    rng: Pool<RngConnectionManager>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            redis: redis_pool(),
            rng: rng_pool(),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let _guard = sentry_init();

    // Get the port number to listen on.
    let port: i32 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    block_on(async {
        // Start a server, configuring the resources to serve.
        let mut app = Server::with_state(State::default());

        app.middleware(RequestLogger::new());
        //     .middleware(Cors::new())
        //     .middleware(Compression::new())
        //     .middleware(Decompression::new());

        app.at("/roll/")
            .get(handlers::parse_roll)
            .post(handlers::roll);

        app.listen(format!("0.0.0.0:{}", port)).await?;
        Ok(())
    })
}
