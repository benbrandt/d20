use actix_web::middleware::cors::Cors;
use actix_web::{http, server, App, HttpResponse, Json, Responder};
use sentry;
use sentry_actix::SentryMiddleware;
use std::env;

mod dice_roller;
use dice_roller::RollInstruction;

fn roll(info: Json<RollInstruction>) -> impl Responder {
    match dice_roller::roll(info.into_inner()) {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(m) => HttpResponse::BadRequest().json(m),
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
                    .resource("/roll/", |r| r.method(http::Method::POST).with(roll))
                    .register()
            })
            .middleware(SentryMiddleware::new())
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|_| panic!("Can not bind to port {:?}", &port))
    .run();
}
