use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &PgPool, plays: &[Play]) -> Result<UpsertStats, sqlx::Error> {
    if plays.is_empty() {
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

    for chunk in plays.chunks(CHUNK_SIZE) {
        let mut tx = pool.begin().await?;

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO plays (
                cfbd_id, cfbd_drive_id, drive_id, game_id,
                drive_number, play_number,
                offense, offense_team_id, offense_conference, offense_score,
                defense, defense_team_id, defense_conference, defense_score,
                home, away, period,
                clock_minutes, clock_seconds,
                offense_timeouts, defense_timeouts,
                yardline, yards_to_goal, down, distance, yards_gained,
                scoring, play_type, play_text, ppa, wallclock
            ) ",
        );

        query_builder.push_values(chunk, |mut b, play| {
            b.push_bind(&play.cfbd_id)
                .push_bind(&play.cfbd_drive_id)
                .push_bind(play.drive_id)
                .push_bind(play.game_id)
                .push_bind(play.drive_number)
                .push_bind(play.play_number)
                .push_bind(&play.offense)
                .push_bind(play.offense_team_id)
                .push_bind(&play.offense_conference)
                .push_bind(play.offense_score)
                .push_bind(&play.defense)
                .push_bind(play.defense_team_id)
                .push_bind(&play.defense_conference)
                .push_bind(play.defense_score)
                .push_bind(&play.home)
                .push_bind(&play.away)
                .push_bind(play.period)
                .push_bind(play.clock_minutes)
                .push_bind(play.clock_seconds)
                .push_bind(play.offense_timeouts)
                .push_bind(play.defense_timeouts)
                .push_bind(play.yardline)
                .push_bind(play.yards_to_goal)
                .push_bind(play.down)
                .push_bind(play.distance)
                .push_bind(play.yards_gained)
                .push_bind(play.scoring)
                .push_bind(&play.play_type)
                .push_bind(&play.play_text)
                .push_bind(play.ppa)
                .push_bind(&play.wallclock);
        });

        query_builder.push(
            " ON CONFLICT (cfbd_id) DO UPDATE SET
                cfbd_drive_id = EXCLUDED.cfbd_drive_id,
                drive_id = EXCLUDED.drive_id,
                game_id = EXCLUDED.game_id,
                drive_number = EXCLUDED.drive_number,
                play_number = EXCLUDED.play_number,
                offense = EXCLUDED.offense,
                offense_team_id = EXCLUDED.offense_team_id,
                offense_conference = EXCLUDED.offense_conference,
                offense_score = EXCLUDED.offense_score,
                defense = EXCLUDED.defense,
                defense_team_id = EXCLUDED.defense_team_id,
                defense_conference = EXCLUDED.defense_conference,
                defense_score = EXCLUDED.defense_score,
                home = EXCLUDED.home,
                away = EXCLUDED.away,
                period = EXCLUDED.period,
                clock_minutes = EXCLUDED.clock_minutes,
                clock_seconds = EXCLUDED.clock_seconds,
                offense_timeouts = EXCLUDED.offense_timeouts,
                defense_timeouts = EXCLUDED.defense_timeouts,
                yardline = EXCLUDED.yardline,
                yards_to_goal = EXCLUDED.yards_to_goal,
                down = EXCLUDED.down,
                distance = EXCLUDED.distance,
                yards_gained = EXCLUDED.yards_gained,
                scoring = EXCLUDED.scoring,
                play_type = EXCLUDED.play_type,
                play_text = EXCLUDED.play_text,
                ppa = EXCLUDED.ppa,
                wallclock = EXCLUDED.wallclock
            WHERE (
                plays.cfbd_drive_id, plays.drive_id, plays.game_id,
                plays.drive_number, plays.play_number,
                plays.offense, plays.offense_team_id, plays.offense_conference, plays.offense_score,
                plays.defense, plays.defense_team_id, plays.defense_conference, plays.defense_score,
                plays.home, plays.away, plays.period,
                plays.clock_minutes, plays.clock_seconds,
                plays.offense_timeouts, plays.defense_timeouts,
                plays.yardline, plays.yards_to_goal, plays.down, plays.distance, plays.yards_gained,
                plays.scoring, plays.play_type, plays.play_text, plays.ppa, plays.wallclock
            ) IS DISTINCT FROM (
                EXCLUDED.cfbd_drive_id, EXCLUDED.drive_id, EXCLUDED.game_id,
                EXCLUDED.drive_number, EXCLUDED.play_number,
                EXCLUDED.offense, EXCLUDED.offense_team_id, EXCLUDED.offense_conference, EXCLUDED.offense_score,
                EXCLUDED.defense, EXCLUDED.defense_team_id, EXCLUDED.defense_conference, EXCLUDED.defense_score,
                EXCLUDED.home, EXCLUDED.away, EXCLUDED.period,
                EXCLUDED.clock_minutes, EXCLUDED.clock_seconds,
                EXCLUDED.offense_timeouts, EXCLUDED.defense_timeouts,
                EXCLUDED.yardline, EXCLUDED.yards_to_goal, EXCLUDED.down, EXCLUDED.distance, EXCLUDED.yards_gained,
                EXCLUDED.scoring, EXCLUDED.play_type, EXCLUDED.play_text, EXCLUDED.ppa, EXCLUDED.wallclock
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
pub struct Play {
    pub id: i32,
    pub cfbd_id: String,
    pub cfbd_drive_id: Option<String>,
    pub drive_id: Option<i32>,
    pub game_id: i32,
    pub drive_number: Option<i32>,
    pub play_number: Option<i32>,
    pub offense: Option<String>,
    pub offense_team_id: i32,
    pub offense_conference: Option<String>,
    pub offense_score: Option<i32>,
    pub defense: Option<String>,
    pub defense_team_id: i32,
    pub defense_conference: Option<String>,
    pub defense_score: Option<i32>,
    pub home: Option<String>,
    pub away: Option<String>,
    pub period: Option<i32>,
    pub clock_minutes: Option<i32>,
    pub clock_seconds: Option<i32>,
    pub offense_timeouts: Option<i32>,
    pub defense_timeouts: Option<i32>,
    pub yardline: Option<i32>,
    pub yards_to_goal: Option<i32>,
    pub down: Option<i32>,
    pub distance: Option<i32>,
    pub yards_gained: Option<i32>,
    pub scoring: Option<bool>,
    pub play_type: Option<String>,
    pub play_text: Option<String>,
    pub ppa: Option<f64>,
    pub wallclock: Option<String>,
}
