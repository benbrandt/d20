use crate::dice_roller::{self, RollInstruction};
use crate::State;
use r2d2_redis::redis::Commands;
use serde::Deserialize;
use std::collections::HashMap;
use tide::{
    error::ResultExt,
    querystring::ContextExt,
    response::{self, IntoResponse},
    Context, EndpointResult,
};

#[derive(Deserialize)]
pub struct RollQuery {
    roll: String,
}

fn roll_to_response(state: &State, instruction: RollInstruction) -> EndpointResult {
    let die = instruction.die;
    let result = dice_roller::roll(instruction).map_err(|e| e.into_response())?;
    // Log stats to redis
    let pool = state.redis.clone();
    let conn = pool.get().server_err()?;
    let mut stats = HashMap::new();
    for roll in &result.rolls {
        *stats.entry(roll).or_insert(0) += 1;
    }
    for (roll, count) in &stats {
        conn.incr(format!("{}:{}", die, roll), *count)
            .server_err()?;
    }
    Ok(response::json(result))
}

pub async fn parse_roll(cx: Context<State>) -> EndpointResult {
    let query: RollQuery = cx.url_query()?;
    let parse_result: EndpointResult<RollInstruction> =
        dice_roller::parse_roll(&query.roll).map_err(|e| e.into_response().into());
    match parse_result {
        Ok(instruction) => roll_to_response(cx.state(), instruction),
        Err(e) => Err(e.into_response().into()),
    }
}

pub async fn roll(mut cx: Context<State>) -> EndpointResult {
    let body = cx.body_json().await.client_err()?;
    roll_to_response(cx.state(), body)
}
