use d20::{establish_connection, models::RollStat, schema::roll_stats};
use diesel::prelude::*;

fn main() {
    let connection = establish_connection();
    let results = roll_stats::table
        .load::<RollStat>(&connection)
        .expect("Error loading stats");

    println!("Displaying {} stats", results.len());
    for stat in results {
        println!("d{:<3} - {:3}: {}", stat.die, stat.roll, stat.roll_count);
    }
}
