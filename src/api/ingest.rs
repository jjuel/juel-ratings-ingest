use crate::api::teams::get_teams_by_year;
use crate::db;
use sqlx::{PgPool, Postgres, Transaction};
use std::fmt;

/// Errors that can bubble up while orchestrating the ingest flow.
#[derive(Debug)]
pub enum IngestError {
    Api(reqwest::Error),
    Database(sqlx::Error),
}

impl fmt::Display for IngestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IngestError::Api(err) => write!(f, "api error: {err}"),
            IngestError::Database(err) => write!(f, "database error: {err}"),
        }
    }
}

impl std::error::Error for IngestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IngestError::Api(err) => Some(err),
            IngestError::Database(err) => Some(err),
        }
    }
}

impl From<reqwest::Error> for IngestError {
    fn from(value: reqwest::Error) -> Self {
        IngestError::Api(value)
    }
}

impl From<sqlx::Error> for IngestError {
    fn from(value: sqlx::Error) -> Self {
        IngestError::Database(value)
    }
}

pub type IngestResult<T> = Result<T, IngestError>;

/// Fetches the CFBD teams API, maps the payload, and persists it via SQLx.
pub async fn ingest_teams_for_year(pool: &PgPool, year: i32) -> IngestResult<()> {
    let teams = get_teams_by_year(year).await?;
    let mut tx = pool.begin().await?;

    for api_team in teams {
        let school = api_team.school.clone();
        let mapped: db::Team = api_team.into(); // conversion lives in db::mappings
        let team_id = upsert_team(&mut tx, mapped).await?;
        println!("upserted team {school} (id={team_id})");
    }

    tx.commit().await?;
    Ok(())
}

async fn upsert_team(tx: &mut Transaction<'_, Postgres>, team: db::Team) -> Result<i32, sqlx::Error> {
    let db::Team {
        id: _,
        cfbd_id,
        school,
        mascot,
        abbreviation,
        conference,
        division,
        classification,
        color,
        alternate_color,
        twitter,
        city,
        state,
        zip,
        country_code,
        timezone,
        latitude,
        longitude,
        elevation,
        capacity,
        construction_year,
        grass,
        dome,
        alternate_names,
    } = team;

    let team_id = sqlx::query_scalar::<_, i32>(
        r#"
        INSERT INTO teams (
            cfbd_id,
            school,
            mascot,
            abbreviation,
            conference,
            division,
            classification,
            color,
            alternate_color,
            twitter,
            city,
            state,
            zip,
            country_code,
            timezone,
            latitude,
            longitude,
            elevation,
            capacity,
            construction_year,
            grass,
            dome,
            alternate_names
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23
        )
        ON CONFLICT (cfbd_id) DO UPDATE SET
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
        RETURNING id
        "#,
    )
    .bind(cfbd_id)
    .bind(school)
    .bind(mascot)
    .bind(abbreviation)
    .bind(conference)
    .bind(division)
    .bind(classification)
    .bind(color)
    .bind(alternate_color)
    .bind(twitter)
    .bind(city)
    .bind(state)
    .bind(zip)
    .bind(country_code)
    .bind(timezone)
    .bind(latitude)
    .bind(longitude)
    .bind(elevation)
    .bind(capacity)
    .bind(construction_year)
    .bind(grass)
    .bind(dome)
    .bind(alternate_names)
    .fetch_one(tx)
    .await?;

    Ok(team_id)
}
