use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &SqlitePool, games: &[Game]) -> Result<UpsertStats, sqlx::Error> {
    if games.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let existing_cfbd_ids: Vec<(i64,)> = {
        let cfbd_ids: Vec<i64> = games.iter().map(|g| g.cfbd_id).collect();
        if cfbd_ids.is_empty() {
            return Ok(UpsertStats {
                ids: vec![],
                inserted: 0,
                updated: 0,
            });
        }
        let placeholders: Vec<&str> = cfbd_ids.iter().map(|_| "?").collect();
        let query = format!(
            "SELECT cfbd_id FROM games WHERE cfbd_id IN ({})",
            placeholders.join(", ")
        );
        let mut query = sqlx::query_as::<_, (i64,)>(&query);
        for id in cfbd_ids {
            query = query.bind(id);
        }
        query.fetch_all(&mut *tx).await?
    };

    let existing_set: std::collections::HashSet<i64> = existing_cfbd_ids.into_iter().map(|(id,)| id).collect();

    let mut all_ids = Vec::new();
    let mut total_inserted = 0;
    let mut total_updated = 0;

    for game in games {
        if existing_set.contains(&game.cfbd_id) {
            total_updated += 1;
        } else {
            total_inserted += 1;
        }

        let id = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO games (cfbd_id, season, week, season_type, start_date, neutral_site, home_team, home_points, away_team, away_points)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(cfbd_id) DO UPDATE SET
                 season = excluded.season,
                 week = excluded.week,
                 season_type = excluded.season_type,
                 start_date = excluded.start_date,
                 neutral_site = excluded.neutral_site,
                 home_team = excluded.home_team,
                 home_points = excluded.home_points,
                 away_team = excluded.away_team,
                 away_points = excluded.away_points
             WHERE games.season IS NOT excluded.season
                OR games.week IS NOT excluded.week
                OR games.season_type IS NOT excluded.season_type
                OR games.start_date IS NOT excluded.start_date
                OR games.neutral_site IS NOT excluded.neutral_site
                OR games.home_team IS NOT excluded.home_team
                OR games.home_points IS NOT excluded.home_points
                OR games.away_team IS NOT excluded.away_team
                OR games.away_points IS NOT excluded.away_points
             RETURNING id"
        )
        .bind(game.cfbd_id)
        .bind(game.season)
        .bind(game.week)
        .bind(&game.season_type)
        .bind(&game.start_date)
        .bind(game.neutral_site)
        .bind(&game.home_team)
        .bind(game.home_points)
        .bind(&game.away_team)
        .bind(game.away_points)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(id) = id {
            all_ids.push(id.0);
        } else if existing_set.contains(&game.cfbd_id) {
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

pub async fn build_game_map(
    pool: &SqlitePool,
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
            sqlx::query_as("SELECT id, cfbd_id FROM games WHERE season = ? AND week = ?")
                .bind(year)
                .bind(w)
                .fetch_all(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT id, cfbd_id FROM games WHERE season = ?")
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
    pub neutral_site: Option<bool>,
    pub home_team: String,
    pub home_points: Option<i32>,
    pub away_team: String,
    pub away_points: Option<i32>,
}
