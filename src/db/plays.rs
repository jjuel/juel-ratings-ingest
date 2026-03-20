use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &SqlitePool, plays: &[Play]) -> Result<UpsertStats, sqlx::Error> {
    const LOOKUP_CHUNK_SIZE: usize = 900;

    if plays.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let existing_set: std::collections::HashSet<String> = {
        let cfbd_ids: Vec<&str> = plays.iter().map(|p| p.cfbd_id.as_str()).collect();
        if cfbd_ids.is_empty() {
            return Ok(UpsertStats {
                ids: vec![],
                inserted: 0,
                updated: 0,
            });
        }

        let mut existing = std::collections::HashSet::new();
        for chunk in cfbd_ids.chunks(LOOKUP_CHUNK_SIZE) {
            let placeholders: Vec<&str> = chunk.iter().map(|_| "?").collect();
            let query = format!(
                "SELECT cfbd_id FROM plays WHERE cfbd_id IN ({})",
                placeholders.join(", ")
            );
            let mut query = sqlx::query_as::<_, (String,)>(&query);
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

    for play in plays {
        if existing_set.contains(&play.cfbd_id) {
            total_updated += 1;
        } else {
            total_inserted += 1;
        }

        let id = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO plays (cfbd_id, cfbd_drive_id, drive_id, game_id, drive_number, play_number, offense, offense_team_id, offense_conference, offense_score, defense, defense_team_id, defense_conference, defense_score, home, away, period, clock_minutes, clock_seconds, offense_timeouts, defense_timeouts, yardline, yards_to_goal, down, distance, yards_gained, scoring, play_type, play_text, ppa, wallclock)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(cfbd_id) DO UPDATE SET
                 cfbd_drive_id = excluded.cfbd_drive_id,
                 drive_id = excluded.drive_id,
                 game_id = excluded.game_id,
                 drive_number = excluded.drive_number,
                 play_number = excluded.play_number,
                 offense = excluded.offense,
                 offense_team_id = excluded.offense_team_id,
                 offense_conference = excluded.offense_conference,
                 offense_score = excluded.offense_score,
                 defense = excluded.defense,
                 defense_team_id = excluded.defense_team_id,
                 defense_conference = excluded.defense_conference,
                 defense_score = excluded.defense_score,
                 home = excluded.home,
                 away = excluded.away,
                 period = excluded.period,
                 clock_minutes = excluded.clock_minutes,
                 clock_seconds = excluded.clock_seconds,
                 offense_timeouts = excluded.offense_timeouts,
                 defense_timeouts = excluded.defense_timeouts,
                 yardline = excluded.yardline,
                 yards_to_goal = excluded.yards_to_goal,
                 down = excluded.down,
                 distance = excluded.distance,
                 yards_gained = excluded.yards_gained,
                 scoring = excluded.scoring,
                 play_type = excluded.play_type,
                 play_text = excluded.play_text,
                 ppa = excluded.ppa,
                 wallclock = excluded.wallclock
             WHERE plays.cfbd_drive_id IS NOT excluded.cfbd_drive_id
                OR plays.drive_id IS NOT excluded.drive_id
                OR plays.game_id IS NOT excluded.game_id
                OR plays.drive_number IS NOT excluded.drive_number
                OR plays.play_number IS NOT excluded.play_number
                OR plays.offense IS NOT excluded.offense
                OR plays.offense_team_id IS NOT excluded.offense_team_id
                OR plays.offense_conference IS NOT excluded.offense_conference
                OR plays.offense_score IS NOT excluded.offense_score
                OR plays.defense IS NOT excluded.defense
                OR plays.defense_team_id IS NOT excluded.defense_team_id
                OR plays.defense_conference IS NOT excluded.defense_conference
                OR plays.defense_score IS NOT excluded.defense_score
                OR plays.home IS NOT excluded.home
                OR plays.away IS NOT excluded.away
                OR plays.period IS NOT excluded.period
                OR plays.clock_minutes IS NOT excluded.clock_minutes
                OR plays.clock_seconds IS NOT excluded.clock_seconds
                OR plays.offense_timeouts IS NOT excluded.offense_timeouts
                OR plays.defense_timeouts IS NOT excluded.defense_timeouts
                OR plays.yardline IS NOT excluded.yardline
                OR plays.yards_to_goal IS NOT excluded.yards_to_goal
                OR plays.down IS NOT excluded.down
                OR plays.distance IS NOT excluded.distance
                OR plays.yards_gained IS NOT excluded.yards_gained
                OR plays.scoring IS NOT excluded.scoring
                OR plays.play_type IS NOT excluded.play_type
                OR plays.play_text IS NOT excluded.play_text
                OR plays.ppa IS NOT excluded.ppa
                OR plays.wallclock IS NOT excluded.wallclock
             RETURNING id"
        )
        .bind(&play.cfbd_id)
        .bind(&play.cfbd_drive_id)
        .bind(play.drive_id)
        .bind(play.game_id)
        .bind(play.drive_number)
        .bind(play.play_number)
        .bind(&play.offense)
        .bind(play.offense_team_id)
        .bind(&play.offense_conference)
        .bind(play.offense_score)
        .bind(&play.defense)
        .bind(play.defense_team_id)
        .bind(&play.defense_conference)
        .bind(play.defense_score)
        .bind(&play.home)
        .bind(&play.away)
        .bind(play.period)
        .bind(play.clock_minutes)
        .bind(play.clock_seconds)
        .bind(play.offense_timeouts)
        .bind(play.defense_timeouts)
        .bind(play.yardline)
        .bind(play.yards_to_goal)
        .bind(play.down)
        .bind(play.distance)
        .bind(play.yards_gained)
        .bind(play.scoring)
        .bind(&play.play_type)
        .bind(&play.play_text)
        .bind(play.ppa)
        .bind(&play.wallclock)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(id) = id {
            all_ids.push(id.0);
        } else if existing_set.contains(&play.cfbd_id) {
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
