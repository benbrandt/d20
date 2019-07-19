use d20::{db_pool, models::RollStat, redis_pool, schema, sentry_init};
use diesel::{self, prelude::*};
use dotenv::dotenv;
use log::debug;
use r2d2_redis::redis::{Commands, Iter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use schema::roll_stats::dsl::{roll_count, roll_stats};

    dotenv().ok();
    let _guard = sentry_init();

    let d_pool = db_pool();
    let d_conn = d_pool.get()?;
    let r_pool = redis_pool();
    let r_conn = r_pool.get()?;

    let keys: Iter<String> = r_conn
        .scan_match("roll_stat:*")
        .expect("Error loading keys");

    for key in keys {
        let die: i16 = key.split(':').next_back().unwrap().parse()?;
        let entries: Vec<String> = r_conn.hgetall(&key)?;

        for chunk in entries.chunks_exact(2) {
            let roll: i16 = chunk.get(0).unwrap().parse()?;
            let value: i64 = chunk.get(1).unwrap().parse()?;
            debug!("{}: {}", roll, value);
            let stat = diesel::update(roll_stats.find((die, roll)))
                .set(roll_count.eq(roll_count + value))
                .get_result::<RollStat>(&d_conn)
                .expect("Error saving value");
            debug!("Stat: {:?}", stat);
        }

        r_conn.del(&key)?;
    }

    Ok(())
}
