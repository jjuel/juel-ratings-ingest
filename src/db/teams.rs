use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &SqlitePool, teams: &[Team]) -> Result<UpsertStats, sqlx::Error> {
    const LOOKUP_CHUNK_SIZE: usize = 900;

    if teams.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let existing_set: HashSet<i32> = {
        let cfbd_ids: Vec<i32> = teams.iter().map(|t| t.cfbd_id).collect();
        if cfbd_ids.is_empty() {
            return Ok(UpsertStats {
                ids: vec![],
                inserted: 0,
                updated: 0,
            });
        }

        let mut existing = HashSet::new();
        for chunk in cfbd_ids.chunks(LOOKUP_CHUNK_SIZE) {
            let placeholders: Vec<&str> = chunk.iter().map(|_| "?").collect();
            let query = format!(
                "SELECT cfbd_id FROM teams WHERE cfbd_id IN ({})",
                placeholders.join(", ")
            );
            let mut query = sqlx::query_as::<_, (i32,)>(&query);
            for id in chunk {
                query = query.bind(*id);
            }

            for (id,) in query.fetch_all(&mut *tx).await? {
                existing.insert(id);
            }
        }

        existing
    };

    let mut all_ids = Vec::new();
    let mut total_inserted = 0;
    let mut total_updated = 0;

    for team in teams {
        if existing_set.contains(&team.cfbd_id) {
            total_updated += 1;
        } else {
            total_inserted += 1;
        }

        let id = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO teams (cfbd_id, school, mascot, abbreviation, conference, division, classification, color, alternate_color, logo_url, alternate_logo_url, twitter, city, state, zip, country_code, timezone, latitude, longitude, elevation, capacity, construction_year, grass, dome, alternate_names)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(cfbd_id) DO UPDATE SET
                 school = excluded.school,
                 mascot = excluded.mascot,
                 abbreviation = excluded.abbreviation,
                 conference = excluded.conference,
                 division = excluded.division,
                 classification = excluded.classification,
                 color = excluded.color,
                 alternate_color = excluded.alternate_color,
                 logo_url = excluded.logo_url,
                 alternate_logo_url = excluded.alternate_logo_url,
                 twitter = excluded.twitter,
                 city = excluded.city,
                 state = excluded.state,
                 zip = excluded.zip,
                 country_code = excluded.country_code,
                 timezone = excluded.timezone,
                 latitude = excluded.latitude,
                 longitude = excluded.longitude,
                 elevation = excluded.elevation,
                 capacity = excluded.capacity,
                 construction_year = excluded.construction_year,
                 grass = excluded.grass,
                 dome = excluded.dome,
                 alternate_names = excluded.alternate_names
             WHERE teams.school IS NOT excluded.school
                OR teams.mascot IS NOT excluded.mascot
                OR teams.abbreviation IS NOT excluded.abbreviation
                OR teams.conference IS NOT excluded.conference
                 OR teams.division IS NOT excluded.division
                 OR teams.classification IS NOT excluded.classification
                 OR teams.color IS NOT excluded.color
                 OR teams.alternate_color IS NOT excluded.alternate_color
                 OR teams.logo_url IS NOT excluded.logo_url
                 OR teams.alternate_logo_url IS NOT excluded.alternate_logo_url
                 OR teams.twitter IS NOT excluded.twitter
                 OR teams.city IS NOT excluded.city
                 OR teams.state IS NOT excluded.state
                OR teams.zip IS NOT excluded.zip
                OR teams.country_code IS NOT excluded.country_code
                OR teams.timezone IS NOT excluded.timezone
                OR teams.latitude IS NOT excluded.latitude
                OR teams.longitude IS NOT excluded.longitude
                OR teams.elevation IS NOT excluded.elevation
                OR teams.capacity IS NOT excluded.capacity
                OR teams.construction_year IS NOT excluded.construction_year
                OR teams.grass IS NOT excluded.grass
                OR teams.dome IS NOT excluded.dome
                OR teams.alternate_names IS NOT excluded.alternate_names
             RETURNING id"
        )
        .bind(team.cfbd_id)
        .bind(&team.school)
        .bind(&team.mascot)
        .bind(&team.abbreviation)
        .bind(&team.conference)
        .bind(&team.division)
        .bind(&team.classification)
        .bind(&team.color)
        .bind(&team.alternate_color)
        .bind(&team.logo_url)
        .bind(&team.alternate_logo_url)
        .bind(&team.twitter)
        .bind(&team.city)
        .bind(&team.state)
        .bind(&team.zip)
        .bind(&team.country_code)
        .bind(&team.timezone)
        .bind(team.latitude)
        .bind(team.longitude)
        .bind(&team.elevation)
        .bind(team.capacity)
        .bind(team.construction_year)
        .bind(team.grass)
        .bind(team.dome)
        .bind(&team.alternate_names)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(id) = id {
            all_ids.push(id.0);
        } else if existing_set.contains(&team.cfbd_id) {
            total_updated -= 1;
        } else {
            total_inserted -= 1;
        }
    }

    tx.commit().await?;

    Ok(UpsertStats {
        ids: all_ids,
        inserted: total_inserted,
        updated: total_updated,
    })
}

pub async fn build_team_name_map(pool: &SqlitePool) -> Result<HashMap<String, i32>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct TeamRow {
        id: i32,
        school: String,
    }

    let rows: Vec<TeamRow> = sqlx::query_as("SELECT id, school FROM teams")
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
    pub logo_url: Option<String>,
    pub alternate_logo_url: Option<String>,
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
    pub alternate_names: Option<String>,
}
