use crate::State;
use d20::{
    dice_roller::{self, RollInstruction},
    REDIS_KEY_ROLL_STATS,
};
use r2d2_redis::redis::pipe;
use serde::Deserialize;
use std::collections::HashMap;
use tide::{http_types::StatusCode, Request, Response, ResultExt};

#[derive(Deserialize)]
pub struct RollQuery {
    roll: String,
}

/// Log stats to redis
pub fn roll_stats(state: &State, die: i32, rolls: &[i32]) {
    let pool = state.redis.clone();
    let mut conn = pool.get().unwrap();
    let mut stat_map = HashMap::new();
    for roll in rolls {
        *stat_map.entry(roll).or_insert(0) += 1;
    }
    let mut pipeline = pipe();
    for (roll, count) in stat_map {
        pipeline.hincr(REDIS_KEY_ROLL_STATS, format!("{}:{}", die, roll), count);
    }
    pipeline.execute(&mut *conn);
}

fn roll_to_response(state: &State, instruction: RollInstruction) -> Response {
    let die = instruction.die;
    let pool = state.rng.clone();
    let mut rng = pool.get().server_err().unwrap();
    let result = dice_roller::roll(&mut *rng, instruction).unwrap();
    roll_stats(state, die, &result.rolls);
    Response::new(StatusCode::Ok).body_json(&result).unwrap()
}

pub async fn parse_roll(cx: Request<State>) -> Response {
    let query: RollQuery = cx.query().unwrap();
    let instruction = dice_roller::parse_roll(&query.roll).unwrap();
    roll_to_response(cx.state(), instruction)
}

pub async fn roll(mut cx: Request<State>) -> Response {
    let body = cx.body_json().await.unwrap();
    roll_to_response(cx.state(), body)
}
