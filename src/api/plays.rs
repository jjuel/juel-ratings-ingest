use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub async fn fetch(year: i32, week: i32, season_type: Option<String>) -> Result<Vec<Play>> {
    let token = get_api_key();
    let client = create_client();

    // Default to 'regular' season type if not provided
    let season_type = season_type.unwrap_or_else(|| "regular".to_string());

    let url = format!("{}plays?year={}&week={}&seasonType={}", CFBD_BASE_URL, year, week, season_type);

    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("Failed to send request to CFBD API")?;

    // Check if response is successful
    let status = response.status();
    let response_text = response.text().await.context("Failed to read response body")?;

    if !status.is_success() {
        eprintln!("API Error ({}): {}", status, response_text);
        return Ok(vec![]);
    }

    // Try to deserialize
    match serde_json::from_str::<Vec<Play>>(&response_text) {
        Ok(plays) => {
            println!("Fetched {} plays", plays.len());
            Ok(plays)
        }
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
            eprintln!("Response (first 500 chars): {}", &response_text[..response_text.len().min(500)]);
            anyhow::bail!("Failed to deserialize plays response: {}", e)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayClock {
    pub seconds: i32,
    pub minutes: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Play {
    pub id: Option<String>,
    pub drive_id: Option<String>,
    pub game_id: Option<i32>,
    pub drive_number: Option<i32>,
    pub play_number: Option<i32>,
    pub offense: Option<String>,
    pub offense_conference: Option<String>,
    pub offense_score: Option<i32>,
    pub defense: Option<String>,
    pub home: Option<String>,
    pub away: Option<String>,
    pub defense_conference: Option<String>,
    pub defense_score: Option<i32>,
    pub period: Option<i32>,
    pub clock: Option<PlayClock>,
    pub offense_timeouts: Option<i32>,
    pub defense_timeouts: Option<i32>,
    pub yardline: Option<i32>,
    pub yards_to_goal: Option<i32>,
    pub down: Option<i32>,
    pub distance: Option<i32>,
    pub yards_gained: Option<i32>,
    pub scoring: Option<bool>,
    pub play_type: Option<String>,
    pub play_text: Option<String>,
    pub ppa: Option<f64>,
    pub wallclock: Option<String>,
}
