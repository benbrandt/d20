use actix_web::middleware::cors::Cors;
use actix_web::{http, server, App, HttpResponse, Query, Responder};
use sentry;
use sentry_actix::SentryMiddleware;
use serde::Deserialize;
use std::env;
use url::percent_encoding::percent_decode;

mod dice_roller;

#[derive(Deserialize)]
struct RollQuery {
    roll: String,
}

fn index(query: Query<RollQuery>) -> impl Responder {
    match dice_roller::roll(
        &percent_decode(query.roll.as_bytes())
            .decode_utf8()
            .unwrap_or_default(),
    ) {
        Ok(r) => HttpResponse::Ok()
            .content_encoding(http::ContentEncoding::Auto)
            .json(r),
        Err(m) => HttpResponse::BadRequest()
            .content_encoding(http::ContentEncoding::Auto)
            .json(m),
    }
}

fn main() {
    let _guard = sentry::init("https://046b94f8170f4135a47ca9d0f9709a6d@sentry.io/1438468");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::env_logger::init(None, Default::default());
    sentry::integrations::panic::register_panic_handler();

    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new()
            .configure(|app| {
                Cors::for_app(app)
                    .resource("/", |r| r.method(http::Method::GET).with(index))
                    .register()
            })
            .middleware(SentryMiddleware::new())
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|_| panic!("Can not bind to port {:?}", &port))
    .run();
}
