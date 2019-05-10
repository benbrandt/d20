use actix_web::{HttpResponse, Json, Query, Responder};
use serde::Deserialize;
use url::percent_encoding::percent_decode;

use crate::dice_roller::{self, RollInstruction};

#[derive(Deserialize)]
pub struct RollQuery {
    roll: String,
}
pub fn parse_roll(query: Query<RollQuery>) -> impl Responder {
    let roll = dice_roller::parse_roll(
        &percent_decode(query.roll.as_bytes())
            .decode_utf8()
            .unwrap_or_default(),
    );
    match roll {
        Ok(r) => match dice_roller::roll(r) {
            Ok(r) => HttpResponse::Ok().json(r),

            Err(m) => HttpResponse::BadRequest().json(m),
        },
        Err(m) => HttpResponse::BadRequest().json(m),
    }
}

pub fn roll(info: Json<RollInstruction>) -> impl Responder {
    match dice_roller::roll(info.into_inner()) {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(m) => HttpResponse::BadRequest().json(m),
    }
}
