use actix_web::middleware::cors::Cors;
use actix_web::{http, server, App, HttpResponse, Query, Responder};
use serde::Deserialize;
use std::env;

mod dice_roller;

#[derive(Deserialize)]
struct RollQuery {
    roll: String,
}

// this handler get called only if the request's query contains `username` field
fn index(query: Query<RollQuery>) -> impl Responder {
    match dice_roller::roll(&query.roll) {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(m) => HttpResponse::BadRequest().json(m),
    }
}

fn main() {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new().configure(|app| {
            Cors::for_app(app)
                .resource("/", |r| r.method(http::Method::GET).with(index))
                .register()
        })
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|_| panic!("Can not bind to port {:?}", &port))
    .run();
}
