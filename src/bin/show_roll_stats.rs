use d20::{db_pool, models::RollStat, schema::roll_stats, sentry_init};
use diesel::prelude::*;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let _guard = sentry_init();
    let pool = db_pool();
    let connection = pool.get().unwrap();
    let results = roll_stats::table
        .load::<RollStat>(&connection)
        .expect("Error loading stats");

    println!("Displaying {} stats", results.len());
    for stat in results {
        println!(
            "d{:<3} - {:3}: {:10} (Updated: {})",
            stat.die, stat.roll, stat.roll_count, stat.updated_at
        );
    }
}
