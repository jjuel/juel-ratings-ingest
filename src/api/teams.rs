use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use crate::cfbd::Team;
use reqwest::Error;

pub async fn get_teams_by_year(year: i32) -> Result<Vec<Team>, Error> {
    let url = format!("{}teams?year={}", CFBD_BASE_URL, year);
    let token = get_api_key();
    let client = create_client();

    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Team>>()
        .await?;

    println!("Fetched {} teams", response.len());
    Ok(response)
}
