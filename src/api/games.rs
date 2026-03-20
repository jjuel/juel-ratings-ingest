use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use reqwest::Error;
use serde::{Deserialize, Serialize};

pub async fn fetch(
    year: i32,
    week: Option<i32>,
    season_type: Option<String>,
) -> Result<Vec<Game>, Error> {
    let token = get_api_key();
    let client = create_client();

    let mut url = format!("{}games?year={}", CFBD_BASE_URL, year);
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
        .json::<Vec<Game>>()
        .await?;

    println!("Fetched {} games", response.len());
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: i64,
    pub season: i32,
    pub week: i32,
    #[serde(rename = "seasonType")]
    pub season_type: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "neutralSite")]
    pub neutral_site: Option<bool>,
    #[serde(rename = "homeTeam")]
    pub home_team: String,
    #[serde(rename = "homePoints")]
    pub home_points: Option<i32>,
    #[serde(rename = "awayTeam")]
    pub away_team: String,
    #[serde(rename = "awayPoints")]
    pub away_points: Option<i32>,
}
