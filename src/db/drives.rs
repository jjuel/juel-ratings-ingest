use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::collections::HashMap;

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &SqlitePool, drives: &[Drive]) -> Result<UpsertStats, sqlx::Error> {
    if drives.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let existing_ids: Vec<(String,)> = {
        let cfbd_ids: Vec<&str> = drives.iter().map(|d| d.cfbd_id.as_str()).collect();
        if cfbd_ids.is_empty() {
            return Ok(UpsertStats {
                ids: vec![],
                inserted: 0,
                updated: 0,
            });
        }
        let placeholders: Vec<&str> = cfbd_ids.iter().map(|_| "?").collect();
        let query = format!(
            "SELECT cfbd_id FROM drives WHERE cfbd_id IN ({})",
            placeholders.join(", ")
        );
        let mut query = sqlx::query_as::<_, (String,)>(&query);
        for id in cfbd_ids {
            query = query.bind(id);
        }
        query.fetch_all(&mut *tx).await?
    };

    let existing_set: std::collections::HashSet<String> = existing_ids.into_iter().map(|(id,)| id).collect();

    let mut all_ids = Vec::new();
    let mut total_inserted = 0;
    let mut total_updated = 0;

    for drive in drives {
        if existing_set.contains(&drive.cfbd_id) {
            total_updated += 1;
        } else {
            total_inserted += 1;
        }

        let id = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO drives (cfbd_id, game_id, offense_team_id, defense_team_id, offense_conference, defense_conference, drive_number, scoring, start_period, start_yardline, start_yards_to_goal, start_time_minutes, start_time_seconds, end_period, end_yardline, end_yards_to_goal, end_time_minutes, end_time_seconds, elapsed_minutes, elapsed_seconds, plays, yards, drive_result, is_home_offense, start_offense_score, start_defense_score, end_offense_score, end_defense_score)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(cfbd_id) DO UPDATE SET
                 game_id = excluded.game_id,
                 offense_team_id = excluded.offense_team_id,
                 defense_team_id = excluded.defense_team_id,
                 offense_conference = excluded.offense_conference,
                 defense_conference = excluded.defense_conference,
                 drive_number = excluded.drive_number,
                 scoring = excluded.scoring,
                 start_period = excluded.start_period,
                 start_yardline = excluded.start_yardline,
                 start_yards_to_goal = excluded.start_yards_to_goal,
                 start_time_minutes = excluded.start_time_minutes,
                 start_time_seconds = excluded.start_time_seconds,
                 end_period = excluded.end_period,
                 end_yardline = excluded.end_yardline,
                 end_yards_to_goal = excluded.end_yards_to_goal,
                 end_time_minutes = excluded.end_time_minutes,
                 end_time_seconds = excluded.end_time_seconds,
                 elapsed_minutes = excluded.elapsed_minutes,
                 elapsed_seconds = excluded.elapsed_seconds,
                 plays = excluded.plays,
                 yards = excluded.yards,
                 drive_result = excluded.drive_result,
                 is_home_offense = excluded.is_home_offense,
                 start_offense_score = excluded.start_offense_score,
                 start_defense_score = excluded.start_defense_score,
                 end_offense_score = excluded.end_offense_score,
                 end_defense_score = excluded.end_defense_score
             WHERE drives.game_id IS NOT excluded.game_id
                OR drives.offense_team_id IS NOT excluded.offense_team_id
                OR drives.defense_team_id IS NOT excluded.defense_team_id
                OR drives.offense_conference IS NOT excluded.offense_conference
                OR drives.defense_conference IS NOT excluded.defense_conference
                OR drives.drive_number IS NOT excluded.drive_number
                OR drives.scoring IS NOT excluded.scoring
                OR drives.start_period IS NOT excluded.start_period
                OR drives.start_yardline IS NOT excluded.start_yardline
                OR drives.start_yards_to_goal IS NOT excluded.start_yards_to_goal
                OR drives.start_time_minutes IS NOT excluded.start_time_minutes
                OR drives.start_time_seconds IS NOT excluded.start_time_seconds
                OR drives.end_period IS NOT excluded.end_period
                OR drives.end_yardline IS NOT excluded.end_yardline
                OR drives.end_yards_to_goal IS NOT excluded.end_yards_to_goal
                OR drives.end_time_minutes IS NOT excluded.end_time_minutes
                OR drives.end_time_seconds IS NOT excluded.end_time_seconds
                OR drives.elapsed_minutes IS NOT excluded.elapsed_minutes
                OR drives.elapsed_seconds IS NOT excluded.elapsed_seconds
                OR drives.plays IS NOT excluded.plays
                OR drives.yards IS NOT excluded.yards
                OR drives.drive_result IS NOT excluded.drive_result
                OR drives.is_home_offense IS NOT excluded.is_home_offense
                OR drives.start_offense_score IS NOT excluded.start_offense_score
                OR drives.start_defense_score IS NOT excluded.start_defense_score
                OR drives.end_offense_score IS NOT excluded.end_offense_score
                OR drives.end_defense_score IS NOT excluded.end_defense_score
             RETURNING id"
        )
        .bind(&drive.cfbd_id)
        .bind(drive.game_id)
        .bind(drive.offense_team_id)
        .bind(drive.defense_team_id)
        .bind(&drive.offense_conference)
        .bind(&drive.defense_conference)
        .bind(drive.drive_number)
        .bind(drive.scoring)
        .bind(drive.start_period)
        .bind(drive.start_yardline)
        .bind(drive.start_yards_to_goal)
        .bind(drive.start_time_minutes)
        .bind(drive.start_time_seconds)
        .bind(drive.end_period)
        .bind(drive.end_yardline)
        .bind(drive.end_yards_to_goal)
        .bind(drive.end_time_minutes)
        .bind(drive.end_time_seconds)
        .bind(drive.elapsed_minutes)
        .bind(drive.elapsed_seconds)
        .bind(drive.plays)
        .bind(drive.yards)
        .bind(&drive.drive_result)
        .bind(drive.is_home_offense)
        .bind(drive.start_offense_score)
        .bind(drive.start_defense_score)
        .bind(drive.end_offense_score)
        .bind(drive.end_defense_score)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(id) = id {
            all_ids.push(id.0);
        } else if existing_set.contains(&drive.cfbd_id) {
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

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Drive {
    pub id: i32,
    pub cfbd_id: String,
    pub game_id: i32,
    pub offense_team_id: i32,
    pub defense_team_id: i32,
    pub offense_conference: Option<String>,
    pub defense_conference: Option<String>,
    pub drive_number: i32,
    pub scoring: Option<bool>,
    pub start_period: Option<i32>,
    pub start_yardline: Option<i32>,
    pub start_yards_to_goal: Option<i32>,
    pub start_time_minutes: Option<i32>,
    pub start_time_seconds: Option<i32>,
    pub end_period: Option<i32>,
    pub end_yardline: Option<i32>,
    pub end_yards_to_goal: Option<i32>,
    pub end_time_minutes: Option<i32>,
    pub end_time_seconds: Option<i32>,
    pub elapsed_minutes: Option<i32>,
    pub elapsed_seconds: Option<i32>,
    pub plays: Option<i32>,
    pub yards: Option<i32>,
    pub drive_result: Option<String>,
    pub is_home_offense: Option<bool>,
    pub start_offense_score: Option<i32>,
    pub start_defense_score: Option<i32>,
    pub end_offense_score: Option<i32>,
    pub end_defense_score: Option<i32>,
}

pub async fn build_drive_map(
    pool: &SqlitePool,
    year: i32,
    week: Option<i32>,
) -> Result<HashMap<String, i32>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct DriveRow {
        id: i32,
        cfbd_id: String,
    }

    let rows: Vec<DriveRow> = match week {
        Some(w) => {
            sqlx::query_as(
                "SELECT d.id, d.cfbd_id
                 FROM drives d
                 JOIN games g ON d.game_id = g.id
                 WHERE g.season = ? AND g.week = ?"
            )
            .bind(year)
            .bind(w)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as(
                "SELECT d.id, d.cfbd_id
                 FROM drives d
                 JOIN games g ON d.game_id = g.id
                 WHERE g.season = ?"
            )
            .bind(year)
            .fetch_all(pool)
            .await?
        }
    };

    Ok(rows.into_iter().map(|row| (row.cfbd_id, row.id)).collect())
}
