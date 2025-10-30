use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &PgPool, games: &[Game]) -> Result<UpsertStats, sqlx::Error> {
    if games.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO games (
            cfbd_id, season, week, season_type, start_date,
            home_team, home_points, away_team, away_points
        ) ",
    );

    query_builder.push_values(games, |mut b, game| {
        b.push_bind(game.cfbd_id)
            .push_bind(game.season)
            .push_bind(game.week)
            .push_bind(&game.season_type)
            .push_bind(game.start_date)
            .push_bind(&game.home_team)
            .push_bind(game.home_points)
            .push_bind(&game.away_team)
            .push_bind(game.away_points);
    });

    query_builder.push(
        " ON CONFLICT (cfbd_id) DO UPDATE SET
            season = EXCLUDED.season,
            week = EXCLUDED.week,
            season_type = EXCLUDED.season_type,
            start_date = EXCLUDED.start_date,
            home_team = EXCLUDED.home_team,
            home_points = EXCLUDED.home_points,
            away_team = EXCLUDED.away_team,
            away_points = EXCLUDED.away_points
        WHERE (
            games.season, games.week, games.season_type, games.start_date,
            games.home_team, games.home_points, games.away_team, games.away_points
        ) IS DISTINCT FROM (
            EXCLUDED.season, EXCLUDED.week, EXCLUDED.season_type, EXCLUDED.start_date,
            EXCLUDED.home_team, EXCLUDED.home_points, EXCLUDED.away_team, EXCLUDED.away_points
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

pub async fn build_game_map(
    pool: &PgPool,
    year: i32,
    week: Option<i32>,
) -> Result<HashMap<i64, i32>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct GameRow {
        id: i32,
        cfbd_id: i64,
    }

    let rows: Vec<GameRow> = match week {
        Some(w) => {
            sqlx::query_as("SELECT id, cfbd_id FROM games WHERE season = $1 AND week = $2")
                .bind(year)
                .bind(w)
                .fetch_all(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT id, cfbd_id FROM games WHERE season = $1")
                .bind(year)
                .fetch_all(pool)
                .await?
        }
    };

    let map: HashMap<i64, i32> = rows.into_iter().map(|row| (row.cfbd_id, row.id)).collect();

    Ok(map)
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub cfbd_id: i64,
    pub season: i32,
    pub week: i32,
    pub season_type: String,
    pub start_date: DateTime<Utc>,
    pub home_team: String,
    pub home_points: Option<i32>,
    pub away_team: String,
    pub away_points: Option<i32>,
}
