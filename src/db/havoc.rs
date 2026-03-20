use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(
    pool: &SqlitePool,
    havoc_stats: &[Havoc],
) -> Result<UpsertStats, sqlx::Error> {
    if havoc_stats.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let existing_keys: Vec<(i32, i32)> = {
        let keys: Vec<(i32, i32)> = havoc_stats.iter().map(|h| (h.game_id, h.team_id)).collect();
        if keys.is_empty() {
            return Ok(UpsertStats {
                ids: vec![],
                inserted: 0,
                updated: 0,
            });
        }
        let mut existing: Vec<(i32, i32)> = Vec::new();
        for &(game_id, team_id) in &keys {
            let row: Option<(i32, i32)> = sqlx::query_as(
                "SELECT game_id, team_id FROM havoc WHERE game_id = ? AND team_id = ?",
            )
            .bind(game_id)
            .bind(team_id)
            .fetch_optional(&mut *tx)
            .await?;
            if let Some(r) = row {
                existing.push(r);
            }
        }
        existing
    };

    let existing_set: std::collections::HashSet<(i32, i32)> = existing_keys.into_iter().collect();

    let mut all_ids = Vec::new();
    let mut total_inserted = 0;
    let mut total_updated = 0;

    for havoc in havoc_stats {
        if existing_set.contains(&(havoc.game_id, havoc.team_id)) {
            total_updated += 1;
        } else {
            total_inserted += 1;
        }

        let id = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO havoc (game_id, team_id, season, season_type, week, team, conference, opponent, opponent_conference, offense_db_havoc_rate, offense_front_seven_havoc_rate, offense_havoc_rate, offense_db_havoc_events, offense_front_seven_havoc_events, offense_total_havoc_events, offense_total_plays, defense_db_havoc_rate, defense_front_seven_havoc_rate, defense_havoc_rate, defense_db_havoc_events, defense_front_seven_havoc_events, defense_total_havoc_events, defense_total_plays)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(game_id, team_id) DO UPDATE SET
                 season = excluded.season,
                 season_type = excluded.season_type,
                 week = excluded.week,
                 team = excluded.team,
                 conference = excluded.conference,
                 opponent = excluded.opponent,
                 opponent_conference = excluded.opponent_conference,
                 offense_db_havoc_rate = excluded.offense_db_havoc_rate,
                 offense_front_seven_havoc_rate = excluded.offense_front_seven_havoc_rate,
                 offense_havoc_rate = excluded.offense_havoc_rate,
                 offense_db_havoc_events = excluded.offense_db_havoc_events,
                 offense_front_seven_havoc_events = excluded.offense_front_seven_havoc_events,
                 offense_total_havoc_events = excluded.offense_total_havoc_events,
                 offense_total_plays = excluded.offense_total_plays,
                 defense_db_havoc_rate = excluded.defense_db_havoc_rate,
                 defense_front_seven_havoc_rate = excluded.defense_front_seven_havoc_rate,
                 defense_havoc_rate = excluded.defense_havoc_rate,
                 defense_db_havoc_events = excluded.defense_db_havoc_events,
                 defense_front_seven_havoc_events = excluded.defense_front_seven_havoc_events,
                 defense_total_havoc_events = excluded.defense_total_havoc_events,
                 defense_total_plays = excluded.defense_total_plays
             WHERE havoc.season IS NOT excluded.season
                OR havoc.season_type IS NOT excluded.season_type
                OR havoc.week IS NOT excluded.week
                OR havoc.team IS NOT excluded.team
                OR havoc.conference IS NOT excluded.conference
                OR havoc.opponent IS NOT excluded.opponent
                OR havoc.opponent_conference IS NOT excluded.opponent_conference
                OR havoc.offense_db_havoc_rate IS NOT excluded.offense_db_havoc_rate
                OR havoc.offense_front_seven_havoc_rate IS NOT excluded.offense_front_seven_havoc_rate
                OR havoc.offense_havoc_rate IS NOT excluded.offense_havoc_rate
                OR havoc.offense_db_havoc_events IS NOT excluded.offense_db_havoc_events
                OR havoc.offense_front_seven_havoc_events IS NOT excluded.offense_front_seven_havoc_events
                OR havoc.offense_total_havoc_events IS NOT excluded.offense_total_havoc_events
                OR havoc.offense_total_plays IS NOT excluded.offense_total_plays
                OR havoc.defense_db_havoc_rate IS NOT excluded.defense_db_havoc_rate
                OR havoc.defense_front_seven_havoc_rate IS NOT excluded.defense_front_seven_havoc_rate
                OR havoc.defense_havoc_rate IS NOT excluded.defense_havoc_rate
                OR havoc.defense_db_havoc_events IS NOT excluded.defense_db_havoc_events
                OR havoc.defense_front_seven_havoc_events IS NOT excluded.defense_front_seven_havoc_events
                OR havoc.defense_total_havoc_events IS NOT excluded.defense_total_havoc_events
                OR havoc.defense_total_plays IS NOT excluded.defense_total_plays
             RETURNING id"
        )
        .bind(havoc.game_id)
        .bind(havoc.team_id)
        .bind(havoc.season)
        .bind(&havoc.season_type)
        .bind(havoc.week)
        .bind(&havoc.team)
        .bind(&havoc.conference)
        .bind(&havoc.opponent)
        .bind(&havoc.opponent_conference)
        .bind(havoc.offense_db_havoc_rate)
        .bind(havoc.offense_front_seven_havoc_rate)
        .bind(havoc.offense_havoc_rate)
        .bind(havoc.offense_db_havoc_events)
        .bind(havoc.offense_front_seven_havoc_events)
        .bind(havoc.offense_total_havoc_events)
        .bind(havoc.offense_total_plays)
        .bind(havoc.defense_db_havoc_rate)
        .bind(havoc.defense_front_seven_havoc_rate)
        .bind(havoc.defense_havoc_rate)
        .bind(havoc.defense_db_havoc_events)
        .bind(havoc.defense_front_seven_havoc_events)
        .bind(havoc.defense_total_havoc_events)
        .bind(havoc.defense_total_plays)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(id) = id {
            all_ids.push(id.0);
        } else if existing_set.contains(&(havoc.game_id, havoc.team_id)) {
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
pub struct Havoc {
    pub id: i32,
    pub game_id: i32,
    pub team_id: i32,
    pub season: i32,
    pub season_type: Option<String>,
    pub week: Option<i32>,
    pub team: String,
    pub conference: Option<String>,
    pub opponent: String,
    pub opponent_conference: Option<String>,
    // Offense stats
    pub offense_db_havoc_rate: Option<f64>,
    pub offense_front_seven_havoc_rate: Option<f64>,
    pub offense_havoc_rate: Option<f64>,
    pub offense_db_havoc_events: Option<f64>,
    pub offense_front_seven_havoc_events: Option<f64>,
    pub offense_total_havoc_events: Option<f64>,
    pub offense_total_plays: Option<f64>,
    // Defense stats
    pub defense_db_havoc_rate: Option<f64>,
    pub defense_front_seven_havoc_rate: Option<f64>,
    pub defense_havoc_rate: Option<f64>,
    pub defense_db_havoc_events: Option<f64>,
    pub defense_front_seven_havoc_events: Option<f64>,
    pub defense_total_havoc_events: Option<f64>,
    pub defense_total_plays: Option<f64>,
}
