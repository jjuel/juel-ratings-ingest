use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use crate::cfbd::GameAdvancedStats;
use reqwest::Error;

pub async fn get_advanced_stats_by_year_and_week(year: i32, week: i32) -> Result<Vec<GameAdvancedStats
>, Error> {
    let url = format!("{}stats/game/advanced?year={}&week={}", CFBD_BASE_URL, year, week);
    let token = get_api_key();
    let client = create_client();

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
