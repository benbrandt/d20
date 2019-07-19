use crate::schema::roll_stats;
use chrono::NaiveDateTime;

#[derive(Debug, Identifiable, Queryable)]
#[primary_key(die, roll)]
pub struct RollStat {
    pub die: i16,
    pub roll: i16,
    pub roll_count: i64,
    pub updated_at: NaiveDateTime,
}
