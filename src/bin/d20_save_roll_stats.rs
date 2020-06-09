#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]
use d20::{db_pool, models::RollStat, redis_pool, schema, sentry_init, REDIS_KEY_ROLL_STATS};
use diesel::{self, prelude::*};
use dotenv::dotenv;
use r2d2_redis::redis::Commands;
use std::error::Error;
use tide::log::{debug, error};

fn main() -> Result<(), Box<dyn Error>> {
    use schema::roll_stats::dsl::{roll_count, roll_stats};

    dotenv().ok();
    let _guard = sentry_init();

    let d_pool = db_pool();
    let d_conn = d_pool.get()?;
    let r_pool = redis_pool();
    let mut r_conn = r_pool.get()?;

    let buffer_key = format!("{}_buffer", REDIS_KEY_ROLL_STATS);

    let rename_success: bool = r_conn.rename_nx(REDIS_KEY_ROLL_STATS, &buffer_key)?;

    if !rename_success {
        error!("Redis error: Key not available");
        return Ok(());
    }

    let entries: Vec<String> = r_conn.hgetall(&buffer_key).expect("Error loading keys");

    for chunk in entries.chunks_exact(2) {
        let mut key = chunk.get(0).unwrap().split(':');
        let die: i16 = key.next().unwrap().parse()?;
        let roll: i16 = key.next().unwrap().parse()?;
        let value: i64 = chunk.get(1).unwrap().parse()?;
        debug!("{}: {}", roll, value);
        let stat = diesel::update(roll_stats.find((die, roll)))
            .set(roll_count.eq(roll_count + value))
            .get_result::<RollStat>(&d_conn)
            .expect("Error saving value");
        debug!("Stat: {:?}", stat);
    }

    r_conn.del(&buffer_key)?;

    Ok(())
}
