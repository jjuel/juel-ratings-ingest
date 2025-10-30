use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &PgPool, teams: &[Team]) -> Result<UpsertStats, sqlx::Error> {
    if teams.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO teams (
            cfbd_id, school, mascot, abbreviation, conference, division,
            classification, color, alternate_color, twitter, city, state,
            zip, country_code, timezone, latitude, longitude, elevation,
            capacity, construction_year, grass, dome, alternate_names
        ) ",
    );

    query_builder.push_values(teams, |mut b, team| {
        b.push_bind(team.cfbd_id)
            .push_bind(&team.school)
            .push_bind(&team.mascot)
            .push_bind(&team.abbreviation)
            .push_bind(&team.conference)
            .push_bind(&team.division)
            .push_bind(&team.classification)
            .push_bind(&team.color)
            .push_bind(&team.alternate_color)
            .push_bind(&team.twitter)
            .push_bind(&team.city)
            .push_bind(&team.state)
            .push_bind(&team.zip)
            .push_bind(&team.country_code)
            .push_bind(&team.timezone)
            .push_bind(team.latitude)
            .push_bind(team.longitude)
            .push_bind(&team.elevation)
            .push_bind(team.capacity)
            .push_bind(team.construction_year)
            .push_bind(team.grass)
            .push_bind(team.dome)
            .push_bind(&team.alternate_names);
    });

    query_builder.push(
        " ON CONFLICT (cfbd_id) DO UPDATE SET
            school = EXCLUDED.school,
            mascot = EXCLUDED.mascot,
            abbreviation = EXCLUDED.abbreviation,
            conference = EXCLUDED.conference,
            division = EXCLUDED.division,
            classification = EXCLUDED.classification,
            color = EXCLUDED.color,
            alternate_color = EXCLUDED.alternate_color,
            twitter = EXCLUDED.twitter,
            city = EXCLUDED.city,
            state = EXCLUDED.state,
            zip = EXCLUDED.zip,
            country_code = EXCLUDED.country_code,
            timezone = EXCLUDED.timezone,
            latitude = EXCLUDED.latitude,
            longitude = EXCLUDED.longitude,
            elevation = EXCLUDED.elevation,
            capacity = EXCLUDED.capacity,
            construction_year = EXCLUDED.construction_year,
            grass = EXCLUDED.grass,
            dome = EXCLUDED.dome,
            alternate_names = EXCLUDED.alternate_names
        WHERE (
            teams.school, teams.mascot, teams.abbreviation, teams.conference,
            teams.division, teams.classification, teams.color, teams.alternate_color,
            teams.twitter, teams.city, teams.state, teams.zip, teams.country_code,
            teams.timezone, teams.latitude, teams.longitude, teams.elevation,
            teams.capacity, teams.construction_year, teams.grass, teams.dome,
            teams.alternate_names
        ) IS DISTINCT FROM (
            EXCLUDED.school, EXCLUDED.mascot, EXCLUDED.abbreviation, EXCLUDED.conference,
            EXCLUDED.division, EXCLUDED.classification, EXCLUDED.color, EXCLUDED.alternate_color,
            EXCLUDED.twitter, EXCLUDED.city, EXCLUDED.state, EXCLUDED.zip, EXCLUDED.country_code,
            EXCLUDED.timezone, EXCLUDED.latitude, EXCLUDED.longitude, EXCLUDED.elevation,
            EXCLUDED.capacity, EXCLUDED.construction_year, EXCLUDED.grass, EXCLUDED.dome,
            EXCLUDED.alternate_names
        )
        RETURNING id, (xmax = 0) AS created",
    );

    let results: Vec<(i32, bool)> = query_builder
        .build_query_as::<(i32, bool)>()
        .fetch_all(&mut *tx)
        .await?;

    tx.commit().await?;

    let ids: Vec<i32> = results.iter().map(|(id, _)| *id).collect();
    let inserted = results.iter().filter(|(_, created)| *created).count();
    let updated = results.len() - inserted;

    Ok(UpsertStats {
        ids,
        inserted,
        updated,
    })
}

pub async fn build_team_name_map(pool: &PgPool) -> Result<HashMap<String, i32>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, school
        FROM teams
        "#
    )
    .fetch_all(pool)
    .await?;

    let map: HashMap<String, i32> = rows.into_iter().map(|row| (row.school, row.id)).collect();

    Ok(map)
}

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
