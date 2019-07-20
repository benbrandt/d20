use crate::dice_roller::{self, RollInstruction};
use crate::State;
use d20::REDIS_KEY_ROLL_STATS;
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

/// Log stats to redis
pub fn roll_stats(state: &State, die: i32, rolls: &[i32]) -> Result<(), failure::Error> {
    let pool = state.redis.clone();
    let mut conn = pool.get()?;
    let mut stats = HashMap::new();
    for roll in rolls {
        *stats.entry(roll).or_insert(0) += 1;
    }
    for (roll, count) in stats {
        conn.hincr(REDIS_KEY_ROLL_STATS, format!("{}:{}", die, roll), count)?;
    }
    Ok(())
}

fn roll_to_response(state: &State, instruction: RollInstruction) -> EndpointResult {
    let die = instruction.die;
    let result = dice_roller::roll(instruction).map_err(|e| e.into_response())?;
    roll_stats(state, die, &result.rolls)
        .map_err(|e| e.compat())
        .server_err()?;
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
