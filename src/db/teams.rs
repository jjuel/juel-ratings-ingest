use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct Team {
    pub id: i32,
    pub cfbd_id: i32,
    pub school: String,
    pub mascot: Option<String>,
    pub abbreviation: Option<String>,
    pub conference: Option<String>,
    pub division: Option<String>,
    pub classification: Option<String>,
    pub color: Option<String>,
    pub alternate_color: Option<String>,
    pub twitter: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<String>,
    pub capacity: Option<i64>,
    pub construction_year: Option<i32>,
    pub grass: Option<bool>,
    pub dome: Option<bool>,
    pub alternate_names: Option<Vec<String>>,
}
