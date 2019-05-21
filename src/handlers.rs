use serde::Deserialize;
use tide::{
    error::ResultExt,
    querystring::ExtractQuery,
    response::{self, IntoResponse},
    Context, EndpointResult,
};

use crate::dice_roller::{self, RollInstruction};

#[derive(Deserialize)]
pub struct RollQuery {
    roll: String,
}

fn roll_to_response(instruction: RollInstruction) -> EndpointResult {
    dice_roller::roll(instruction)
        .map(response::json)
        .map_err(|e| e.into_response().into())
}

pub async fn parse_roll(cx: Context<()>) -> EndpointResult {
    let query: RollQuery = cx.url_query()?;
    let parse_result: EndpointResult<RollInstruction> =
        dice_roller::parse_roll(&query.roll).map_err(|e| e.into_response().into());
    match parse_result {
        Ok(instruction) => roll_to_response(instruction),
        Err(e) => Err(e.into_response().into()),
    }
}

pub async fn roll(mut cx: Context<()>) -> EndpointResult {
    roll_to_response(cx.body_json().await.client_err()?)
}
