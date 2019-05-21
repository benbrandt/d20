#![feature(async_await)]
#![warn(clippy::all)]
use sentry;
use std::env;
use tide::{
    middleware::{DefaultHeaders, RootLogger},
    App,
};

mod dice_roller;
mod handlers;

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
    let mut app = App::new(());

    app.middleware(RootLogger::new()).middleware(
        DefaultHeaders::new()
            .header("Access-Control-Allow-Origin", "*")
            .header(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, PATCH, DELETE, OPTIONS",
            ),
    );

    app.at("/roll/")
        .get(handlers::parse_roll)
        .post(handlers::roll);

    app.serve(format!("0.0.0.0:{}", port))
        .unwrap_or_else(|_| panic!("Can not bind to port {:?}", &port));
}
