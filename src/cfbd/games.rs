use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: i64,
    pub season: i32,
    pub week: i32,
    #[serde(rename = "seasonType")]
    pub season_type: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "homeTeam")]
    pub home_team: String,
    #[serde(rename = "homePoints")]
    pub home_points: Option<i32>,
    #[serde(rename = "awayTeam")]
    pub away_team: String,
    #[serde(rename = "awayPoints")]
    pub away_points: Option<i32>,
}
