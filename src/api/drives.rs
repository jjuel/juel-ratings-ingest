use crate::api::CFBD_BASE_URL;
use crate::api::client::{create_client, get_api_key};
use crate::cfbd::drives::Drive;
use reqwest::Error;

pub async fn get_drives_by_year_and_week(year: i32, week: i32) -> Result<Vec<Drive>, Error> {
    let url = format!("{}drives?year={}&week={}", CFBD_BASE_URL, year, week);
    let token = get_api_key();
    let client = create_client();

    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Vec<Drive>>()
        .await?;

    println!("Fetched {} drives", response.len());
    Ok(response)
}
