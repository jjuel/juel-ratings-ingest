use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub cfbd_id: i64,
    pub season: i32,
    pub week: i32,
    pub season_type: String,
    pub start_date: DateTime<Utc>,
    pub home_team: String,
    pub home_points: Option<i32>,
    pub away_team: String,
    pub away_points: Option<i32>,
}
