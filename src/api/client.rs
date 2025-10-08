use dotenv::dotenv;
use reqwest::Client;
use std::env;

pub fn create_client() -> Client {
    Client::new()
}

pub fn get_api_key() -> String {
    dotenv().ok();
    env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set")
}
