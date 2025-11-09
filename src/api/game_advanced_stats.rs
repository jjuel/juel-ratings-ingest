use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use reqwest::Error;
use serde::{Deserialize, Serialize};

pub async fn fetch(year: i32, week: Option<i32>, season_type: Option<String>) -> Result<Vec<GameAdvancedStats>, Error> {
    let token = get_api_key();
    let client = create_client();

    let mut url = format!("{}stats/game/advanced?year={}", CFBD_BASE_URL, year);
    if let Some(w) = week {
        url.push_str(&format!("&week={}", w));
    }

    if let Some(st) = season_type {
        url.push_str(&format!("&seasonType={}", st));
    }

    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<GameAdvancedStats>>()
        .await?;

    println!("Fetched advanced stats for {} games", response.len());
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameAdvancedStats {
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub season: i32,
    pub season_type: String,
    pub week: i32,
    pub team: String,
    pub opponent: String,
    pub offense: UnitAdvancedStats,
    pub defense: UnitAdvancedStats,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitAdvancedStats {
    pub passing_plays: PlaySplitStats,
    pub rushing_plays: PlaySplitStats,
    pub passing_downs: EfficiencyStats,
    pub standard_downs: EfficiencyStats,
    pub open_field_yards_total: Option<f64>,
    pub open_field_yards: Option<f64>,
    pub second_level_yards_total: Option<f64>,
    pub second_level_yards: Option<f64>,
    pub line_yards_total: Option<f64>,
    pub line_yards: Option<f64>,
    pub stuff_rate: Option<f64>,
    pub power_success: Option<f64>,
    pub explosiveness: Option<f64>,
    pub success_rate: Option<f64>,
    #[serde(rename = "totalPPA")]
    pub total_ppa: Option<f64>,
    pub ppa: Option<f64>,
    pub drives: Option<i32>,
    pub plays: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaySplitStats {
    pub explosiveness: Option<f64>,
    pub success_rate: Option<f64>,
    #[serde(rename = "totalPPA")]
    pub total_ppa: Option<f64>,
    pub ppa: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EfficiencyStats {
    pub explosiveness: Option<f64>,
    pub success_rate: Option<f64>,
    pub ppa: Option<f64>,
}
