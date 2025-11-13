use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use reqwest::Error;
use serde::{Deserialize, Serialize};

pub async fn fetch(year: i32, week: Option<i32>, season_type: Option<String>) -> Result<Vec<Havoc>, Error> {
    let token = get_api_key();
    let client = create_client();

    let mut url = format!("{}stats/game/havoc?year={}", CFBD_BASE_URL, year);
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
        .json::<Vec<Havoc>>()
        .await?;

    println!("Fetched havoc stats for {} games", response.len());
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HavocStats {
    pub db_havoc_rate: Option<f64>,
    pub front_seven_havoc_rate: Option<f64>,
    pub havoc_rate: Option<f64>,
    pub db_havoc_events: Option<f64>,
    pub front_seven_havoc_events: Option<f64>,
    pub total_havoc_events: Option<f64>,
    pub total_plays: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Havoc {
    pub game_id: i64,
    pub season: i32,
    pub season_type: Option<String>,
    pub week: Option<i32>,
    pub team: String,
    pub conference: Option<String>,
    pub opponent: String,
    pub opponent_conference: Option<String>,
    pub offense: Option<HavocStats>,
    pub defense: Option<HavocStats>,
}
