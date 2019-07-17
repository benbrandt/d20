use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable)]
pub struct RollStat {
    pub die: i16,
    pub roll: i16,
    pub roll_count: i64,
    pub updated_at: NaiveDateTime,
}
