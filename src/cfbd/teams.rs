use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    pub id: i32,
    pub school: String,
    pub mascot: Option<String>,
    pub abbreviation: Option<String>,
    pub conference: Option<String>,
    pub division: Option<String>,
    pub classification: Option<String>,
    pub color: Option<String>,
    #[serde(rename = "alternateColor")]
    pub alternate_color: Option<String>,
    pub twitter: Option<String>,

    #[serde(default)]
    pub location: Option<TeamLocation>,

    #[serde(rename = "altNames")]
    pub alternate_names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamLocation {
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<String>,
    pub capacity: Option<i64>,
    #[serde(rename = "constructionYear")]
    pub construction_year: Option<i32>,
    pub grass: Option<bool>,
    pub dome: Option<bool>,
}
