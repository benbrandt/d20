use crate::State;
use d20::{
    dice_roller::{self, RollInstruction},
    REDIS_KEY_ROLL_STATS,
};
use r2d2_redis::redis::pipe;
use serde::Deserialize;
use std::collections::HashMap;
use tide::{prelude::json, Request};

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

fn roll_to_response(state: &State, instruction: RollInstruction) -> tide::Result {
    let die = instruction.die;
    let pool = state.rng.clone();
    let mut rng = pool.get()?;
    let result = dice_roller::roll(&mut *rng, instruction)?;
    roll_stats(state, die, &result.rolls);
    Ok(json!(&result).into())
}

pub async fn parse_roll(req: Request<State>) -> tide::Result {
    let query: RollQuery = req.query()?;
    let instruction = dice_roller::parse_roll(&query.roll)?;
    roll_to_response(req.state(), instruction)
}

pub async fn roll(mut req: Request<State>) -> tide::Result {
    let body = req.body_json().await?;
    roll_to_response(req.state(), body)
}
