use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Drive {
    pub offense: String,
    #[serde(default)]
    pub offense_conference: Option<String>,
    pub defense: String,
    #[serde(default)]
    pub defense_conference: Option<String>,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub id: String,
    #[serde(rename = "driveNumber")]
    pub drive_number: i32,
    pub scoring: Option<bool>,
    #[serde(rename = "startPeriod")]
    pub start_period: Option<i32>,
    #[serde(rename = "startYardline")]
    pub start_yardline: Option<i32>,
    #[serde(rename = "startYardsToGoal")]
    pub start_yards_to_goal: Option<i32>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: Option<DriveClock>,
    #[serde(rename = "endPeriod")]
    pub end_period: Option<i32>,
    #[serde(rename = "endYardline")]
    pub end_yardline: Option<i32>,
    #[serde(rename = "endYardsToGoal")]
    pub end_yards_to_goal: Option<i32>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: Option<DriveClock>,
    #[serde(default)]
    pub elapsed: Option<DriveClock>,
    pub plays: Option<i32>,
    pub yards: Option<i32>,
    #[serde(rename = "driveResult")]
    pub drive_result: Option<String>,
    #[serde(rename = "isHomeOffense")]
    pub is_home_offense: Option<bool>,
    #[serde(rename = "startOffenseScore")]
    pub start_offense_score: Option<i32>,
    #[serde(rename = "startDefenseScore")]
    pub start_defense_score: Option<i32>,
    #[serde(rename = "endOffenseScore")]
    pub end_offense_score: Option<i32>,
    #[serde(rename = "endDefenseScore")]
    pub end_defense_score: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveClock {
    pub seconds: Option<i32>,
    pub minutes: Option<i32>,
}
