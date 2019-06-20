#![feature(async_await)]
#![warn(clippy::all)]
use http_service::Body;
use sentry;
use std::env;
use tide::{
    http::{header, status::StatusCode, Response},
    middleware::RootLogger,
    App,
};

mod cors;
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
    let mut app = App::new(State::default());

    app.middleware(RootLogger::new())
        .middleware(cors::CorsBlanket::new());

    app.at("/graphql").post(graphql::handle_graphql);
    app.at("/graphiql").get(graphql::handle_graphiql);

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

    app.serve(format!("0.0.0.0:{}", port))
        .unwrap_or_else(|_| panic!("Can not bind to port {:?}", &port));
}
