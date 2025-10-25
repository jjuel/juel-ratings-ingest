use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use reqwest::Error;
use serde::{Deserialize, Serialize};

pub async fn fetch(year: u32, week: u32) -> Result<Vec<Game>, Error> {
    let url = format!("{}games?year={}&week={}", CFBD_BASE_URL, year, week);
    let token = get_api_key();
    let client = create_client();

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
    #[serde(rename = "homeTeam")]
    pub home_team: String,
    #[serde(rename = "homePoints")]
    pub home_points: Option<i32>,
    #[serde(rename = "awayTeam")]
    pub away_team: String,
    #[serde(rename = "awayPoints")]
    pub away_points: Option<i32>,
}
