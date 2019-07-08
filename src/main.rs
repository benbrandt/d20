#![feature(async_await, async_closure)]
#![warn(clippy::all)]
use http_service::Body;
use sentry;
use std::env;
use tide::{
    http::{header, status::StatusCode, Response},
    middleware::{CorsMiddleware, RequestLogger},
    App,
};
use tide_compression::{Compression, Decompression};

mod dice_roller;
mod graphql;
mod handlers;

// First, we define `State` that holds accumulator state. This is accessible as state in
// Tide, and as executor context in Juniper.
#[derive(Clone, Default)]
pub struct State(());

fn main() {
    let _guard = sentry::init("https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::env_logger::init(None, Default::default());
    sentry::integrations::panic::register_panic_handler();

    // Get the port number to listen on.
    let port: i32 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    let mut app = App::with_state(State::default());

    app.middleware(RequestLogger::new())
        .middleware(CorsMiddleware::new())
        .middleware(Compression::new())
        .middleware(Decompression::new());

    app.at("/graphql").post(graphql::handle_graphql);
    #[cfg(debug_assertions)]
    app.at("/graphiql").get(graphql::handle_graphiql);
    #[cfg(debug_assertions)]
    app.at("/schema").get(graphql::handle_schema);

    app.at("/roll/")
        .get(handlers::parse_roll)
        .options(async move |_| {
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime::TEXT_PLAIN.as_ref())
                .body(Body::empty())
                .unwrap()
        })
        .post(handlers::roll);

    app.run(format!("0.0.0.0:{}", port)).unwrap();
}
