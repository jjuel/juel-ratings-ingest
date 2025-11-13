use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(pool: &PgPool, havoc_stats: &[Havoc]) -> Result<UpsertStats, sqlx::Error> {
    if havoc_stats.is_empty() {
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

    for chunk in havoc_stats.chunks(CHUNK_SIZE) {
        let mut tx = pool.begin().await?;

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO havoc (
                game_id, team_id, season, season_type, week,
                team, conference, opponent, opponent_conference,
                offense_db_havoc_rate, offense_front_seven_havoc_rate, offense_havoc_rate,
                offense_db_havoc_events, offense_front_seven_havoc_events, offense_total_havoc_events, offense_total_plays,
                defense_db_havoc_rate, defense_front_seven_havoc_rate, defense_havoc_rate,
                defense_db_havoc_events, defense_front_seven_havoc_events, defense_total_havoc_events, defense_total_plays
            ) ",
        );

        query_builder.push_values(chunk, |mut b, havoc| {
            b.push_bind(havoc.game_id)
                .push_bind(havoc.team_id)
                .push_bind(havoc.season)
                .push_bind(&havoc.season_type)
                .push_bind(havoc.week)
                .push_bind(&havoc.team)
                .push_bind(&havoc.conference)
                .push_bind(&havoc.opponent)
                .push_bind(&havoc.opponent_conference)
                .push_bind(havoc.offense_db_havoc_rate)
                .push_bind(havoc.offense_front_seven_havoc_rate)
                .push_bind(havoc.offense_havoc_rate)
                .push_bind(havoc.offense_db_havoc_events)
                .push_bind(havoc.offense_front_seven_havoc_events)
                .push_bind(havoc.offense_total_havoc_events)
                .push_bind(havoc.offense_total_plays)
                .push_bind(havoc.defense_db_havoc_rate)
                .push_bind(havoc.defense_front_seven_havoc_rate)
                .push_bind(havoc.defense_havoc_rate)
                .push_bind(havoc.defense_db_havoc_events)
                .push_bind(havoc.defense_front_seven_havoc_events)
                .push_bind(havoc.defense_total_havoc_events)
                .push_bind(havoc.defense_total_plays);
        });

        query_builder.push(
            " ON CONFLICT (game_id, team_id) DO UPDATE SET
                season = EXCLUDED.season,
                season_type = EXCLUDED.season_type,
                week = EXCLUDED.week,
                team = EXCLUDED.team,
                conference = EXCLUDED.conference,
                opponent = EXCLUDED.opponent,
                opponent_conference = EXCLUDED.opponent_conference,
                offense_db_havoc_rate = EXCLUDED.offense_db_havoc_rate,
                offense_front_seven_havoc_rate = EXCLUDED.offense_front_seven_havoc_rate,
                offense_havoc_rate = EXCLUDED.offense_havoc_rate,
                offense_db_havoc_events = EXCLUDED.offense_db_havoc_events,
                offense_front_seven_havoc_events = EXCLUDED.offense_front_seven_havoc_events,
                offense_total_havoc_events = EXCLUDED.offense_total_havoc_events,
                offense_total_plays = EXCLUDED.offense_total_plays,
                defense_db_havoc_rate = EXCLUDED.defense_db_havoc_rate,
                defense_front_seven_havoc_rate = EXCLUDED.defense_front_seven_havoc_rate,
                defense_havoc_rate = EXCLUDED.defense_havoc_rate,
                defense_db_havoc_events = EXCLUDED.defense_db_havoc_events,
                defense_front_seven_havoc_events = EXCLUDED.defense_front_seven_havoc_events,
                defense_total_havoc_events = EXCLUDED.defense_total_havoc_events,
                defense_total_plays = EXCLUDED.defense_total_plays
            WHERE (
                havoc.season, havoc.season_type, havoc.week,
                havoc.team, havoc.conference, havoc.opponent, havoc.opponent_conference,
                havoc.offense_db_havoc_rate, havoc.offense_front_seven_havoc_rate, havoc.offense_havoc_rate,
                havoc.offense_db_havoc_events, havoc.offense_front_seven_havoc_events,
                havoc.offense_total_havoc_events, havoc.offense_total_plays,
                havoc.defense_db_havoc_rate, havoc.defense_front_seven_havoc_rate, havoc.defense_havoc_rate,
                havoc.defense_db_havoc_events, havoc.defense_front_seven_havoc_events,
                havoc.defense_total_havoc_events, havoc.defense_total_plays
            ) IS DISTINCT FROM (
                EXCLUDED.season, EXCLUDED.season_type, EXCLUDED.week,
                EXCLUDED.team, EXCLUDED.conference, EXCLUDED.opponent, EXCLUDED.opponent_conference,
                EXCLUDED.offense_db_havoc_rate, EXCLUDED.offense_front_seven_havoc_rate, EXCLUDED.offense_havoc_rate,
                EXCLUDED.offense_db_havoc_events, EXCLUDED.offense_front_seven_havoc_events,
                EXCLUDED.offense_total_havoc_events, EXCLUDED.offense_total_plays,
                EXCLUDED.defense_db_havoc_rate, EXCLUDED.defense_front_seven_havoc_rate, EXCLUDED.defense_havoc_rate,
                EXCLUDED.defense_db_havoc_events, EXCLUDED.defense_front_seven_havoc_events,
                EXCLUDED.defense_total_havoc_events, EXCLUDED.defense_total_plays
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
