use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &PgPool, drives: &[Drive]) -> Result<UpsertStats, sqlx::Error> {
    if drives.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    const CHUNK_SIZE: usize = 1000;
    let mut all_ids = Vec::new();
    let mut total_inserted = 0;
    let mut total_updated = 0;

    for chunk in drives.chunks(CHUNK_SIZE) {
        let mut tx = pool.begin().await?;

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO drives (
                cfbd_id, game_id, offense_team_id, defense_team_id,
                offense_conference, defense_conference, drive_number, scoring,
                start_period, start_yardline, start_yards_to_goal,
                start_time_minutes, start_time_seconds,
                end_period, end_yardline, end_yards_to_goal,
                end_time_minutes, end_time_seconds,
                elapsed_minutes, elapsed_seconds,
                plays, yards, drive_result,
                is_home_offense, start_offense_score, start_defense_score,
                end_offense_score, end_defense_score
            ) ",
        );

        query_builder.push_values(chunk, |mut b, drive| {
            b.push_bind(&drive.cfbd_id)
                .push_bind(drive.game_id)
                .push_bind(drive.offense_team_id)
                .push_bind(drive.defense_team_id)
                .push_bind(&drive.offense_conference)
                .push_bind(&drive.defense_conference)
                .push_bind(drive.drive_number)
                .push_bind(drive.scoring)
                .push_bind(drive.start_period)
                .push_bind(drive.start_yardline)
                .push_bind(drive.start_yards_to_goal)
                .push_bind(drive.start_time_minutes)
                .push_bind(drive.start_time_seconds)
                .push_bind(drive.end_period)
                .push_bind(drive.end_yardline)
                .push_bind(drive.end_yards_to_goal)
                .push_bind(drive.end_time_minutes)
                .push_bind(drive.end_time_seconds)
                .push_bind(drive.elapsed_minutes)
                .push_bind(drive.elapsed_seconds)
                .push_bind(drive.plays)
                .push_bind(drive.yards)
                .push_bind(&drive.drive_result)
                .push_bind(drive.is_home_offense)
                .push_bind(drive.start_offense_score)
                .push_bind(drive.start_defense_score)
                .push_bind(drive.end_offense_score)
                .push_bind(drive.end_defense_score);
        });

        query_builder.push(
            " ON CONFLICT (cfbd_id) DO UPDATE SET
                game_id = EXCLUDED.game_id,
                offense_team_id = EXCLUDED.offense_team_id,
                defense_team_id = EXCLUDED.defense_team_id,
                offense_conference = EXCLUDED.offense_conference,
                defense_conference = EXCLUDED.defense_conference,
                drive_number = EXCLUDED.drive_number,
                scoring = EXCLUDED.scoring,
                start_period = EXCLUDED.start_period,
                start_yardline = EXCLUDED.start_yardline,
                start_yards_to_goal = EXCLUDED.start_yards_to_goal,
                start_time_minutes = EXCLUDED.start_time_minutes,
                start_time_seconds = EXCLUDED.start_time_seconds,
                end_period = EXCLUDED.end_period,
                end_yardline = EXCLUDED.end_yardline,
                end_yards_to_goal = EXCLUDED.end_yards_to_goal,
                end_time_minutes = EXCLUDED.end_time_minutes,
                end_time_seconds = EXCLUDED.end_time_seconds,
                elapsed_minutes = EXCLUDED.elapsed_minutes,
                elapsed_seconds = EXCLUDED.elapsed_seconds,
                plays = EXCLUDED.plays,
                yards = EXCLUDED.yards,
                drive_result = EXCLUDED.drive_result,
                is_home_offense = EXCLUDED.is_home_offense,
                start_offense_score = EXCLUDED.start_offense_score,
                start_defense_score = EXCLUDED.start_defense_score,
                end_offense_score = EXCLUDED.end_offense_score,
                end_defense_score = EXCLUDED.end_defense_score
            WHERE (
                drives.game_id, drives.offense_team_id, drives.defense_team_id,
                drives.offense_conference, drives.defense_conference, drives.drive_number,
                drives.scoring, drives.start_period, drives.start_yardline,
                drives.start_yards_to_goal, drives.start_time_minutes, drives.start_time_seconds,
                drives.end_period, drives.end_yardline, drives.end_yards_to_goal,
                drives.end_time_minutes, drives.end_time_seconds, drives.elapsed_minutes,
                drives.elapsed_seconds, drives.plays, drives.yards, drives.drive_result,
                drives.is_home_offense, drives.start_offense_score, drives.start_defense_score,
                drives.end_offense_score, drives.end_defense_score
            ) IS DISTINCT FROM (
                EXCLUDED.game_id, EXCLUDED.offense_team_id, EXCLUDED.defense_team_id,
                EXCLUDED.offense_conference, EXCLUDED.defense_conference, EXCLUDED.drive_number,
                EXCLUDED.scoring, EXCLUDED.start_period, EXCLUDED.start_yardline,
                EXCLUDED.start_yards_to_goal, EXCLUDED.start_time_minutes, EXCLUDED.start_time_seconds,
                EXCLUDED.end_period, EXCLUDED.end_yardline, EXCLUDED.end_yards_to_goal,
                EXCLUDED.end_time_minutes, EXCLUDED.end_time_seconds, EXCLUDED.elapsed_minutes,
                EXCLUDED.elapsed_seconds, EXCLUDED.plays, EXCLUDED.yards, EXCLUDED.drive_result,
                EXCLUDED.is_home_offense, EXCLUDED.start_offense_score, EXCLUDED.start_defense_score,
                EXCLUDED.end_offense_score, EXCLUDED.end_defense_score
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

        all_ids.extend(ids);
        total_inserted += inserted;
        total_updated += updated;
    }

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
