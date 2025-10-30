use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(
    pool: &PgPool,
    stats: &[GameAdvancedStats],
) -> Result<UpsertStats, sqlx::Error> {
    if stats.is_empty() {
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

    for chunk in stats.chunks(CHUNK_SIZE) {
        let mut tx = pool.begin().await?;

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO game_advanced_stats (
                game_id, team_id, opponent_id,
                offense_passing_plays_explosiveness, offense_passing_plays_success_rate,
                offense_passing_plays_total_ppa, offense_passing_plays_ppa,
                offense_rushing_plays_explosiveness, offense_rushing_plays_success_rate,
                offense_rushing_plays_total_ppa, offense_rushing_plays_ppa,
                offense_passing_downs_explosiveness, offense_passing_downs_success_rate,
                offense_passing_downs_ppa,
                offense_standard_downs_explosiveness, offense_standard_downs_success_rate,
                offense_standard_downs_ppa,
                offense_open_field_yards_total, offense_open_field_yards,
                offense_second_level_yards_total, offense_second_level_yards,
                offense_line_yards_total, offense_line_yards,
                offense_stuff_rate, offense_power_success,
                offense_explosiveness, offense_success_rate,
                offense_total_ppa, offense_ppa,
                offense_drives, offense_plays,
                defense_passing_plays_explosiveness, defense_passing_plays_success_rate,
                defense_passing_plays_total_ppa, defense_passing_plays_ppa,
                defense_rushing_plays_explosiveness, defense_rushing_plays_success_rate,
                defense_rushing_plays_total_ppa, defense_rushing_plays_ppa,
                defense_passing_downs_explosiveness, defense_passing_downs_success_rate,
                defense_passing_downs_ppa,
                defense_standard_downs_explosiveness, defense_standard_downs_success_rate,
                defense_standard_downs_ppa,
                defense_open_field_yards_total, defense_open_field_yards,
                defense_second_level_yards_total, defense_second_level_yards,
                defense_line_yards_total, defense_line_yards,
                defense_stuff_rate, defense_power_success,
                defense_explosiveness, defense_success_rate,
                defense_total_ppa, defense_ppa,
                defense_drives, defense_plays,
                raw
            ) ",
        );

        query_builder.push_values(chunk, |mut b, stat| {
            b.push_bind(stat.game_id)
                .push_bind(stat.team_id)
                .push_bind(stat.opponent_id)
                .push_bind(stat.offense_passing_plays_explosiveness)
                .push_bind(stat.offense_passing_plays_success_rate)
                .push_bind(stat.offense_passing_plays_total_ppa)
                .push_bind(stat.offense_passing_plays_ppa)
                .push_bind(stat.offense_rushing_plays_explosiveness)
                .push_bind(stat.offense_rushing_plays_success_rate)
                .push_bind(stat.offense_rushing_plays_total_ppa)
                .push_bind(stat.offense_rushing_plays_ppa)
                .push_bind(stat.offense_passing_downs_explosiveness)
                .push_bind(stat.offense_passing_downs_success_rate)
                .push_bind(stat.offense_passing_downs_ppa)
                .push_bind(stat.offense_standard_downs_explosiveness)
                .push_bind(stat.offense_standard_downs_success_rate)
                .push_bind(stat.offense_standard_downs_ppa)
                .push_bind(stat.offense_open_field_yards_total)
                .push_bind(stat.offense_open_field_yards)
                .push_bind(stat.offense_second_level_yards_total)
                .push_bind(stat.offense_second_level_yards)
                .push_bind(stat.offense_line_yards_total)
                .push_bind(stat.offense_line_yards)
                .push_bind(stat.offense_stuff_rate)
                .push_bind(stat.offense_power_success)
                .push_bind(stat.offense_explosiveness)
                .push_bind(stat.offense_success_rate)
                .push_bind(stat.offense_total_ppa)
                .push_bind(stat.offense_ppa)
                .push_bind(stat.offense_drives)
                .push_bind(stat.offense_plays)
                .push_bind(stat.defense_passing_plays_explosiveness)
                .push_bind(stat.defense_passing_plays_success_rate)
                .push_bind(stat.defense_passing_plays_total_ppa)
                .push_bind(stat.defense_passing_plays_ppa)
                .push_bind(stat.defense_rushing_plays_explosiveness)
                .push_bind(stat.defense_rushing_plays_success_rate)
                .push_bind(stat.defense_rushing_plays_total_ppa)
                .push_bind(stat.defense_rushing_plays_ppa)
                .push_bind(stat.defense_passing_downs_explosiveness)
                .push_bind(stat.defense_passing_downs_success_rate)
                .push_bind(stat.defense_passing_downs_ppa)
                .push_bind(stat.defense_standard_downs_explosiveness)
                .push_bind(stat.defense_standard_downs_success_rate)
                .push_bind(stat.defense_standard_downs_ppa)
                .push_bind(stat.defense_open_field_yards_total)
                .push_bind(stat.defense_open_field_yards)
                .push_bind(stat.defense_second_level_yards_total)
                .push_bind(stat.defense_second_level_yards)
                .push_bind(stat.defense_line_yards_total)
                .push_bind(stat.defense_line_yards)
                .push_bind(stat.defense_stuff_rate)
                .push_bind(stat.defense_power_success)
                .push_bind(stat.defense_explosiveness)
                .push_bind(stat.defense_success_rate)
                .push_bind(stat.defense_total_ppa)
                .push_bind(stat.defense_ppa)
                .push_bind(stat.defense_drives)
                .push_bind(stat.defense_plays)
                .push_bind(&stat.raw);
        });

        query_builder.push(
            " ON CONFLICT (game_id, team_id) DO UPDATE SET
                opponent_id = EXCLUDED.opponent_id,
                offense_passing_plays_explosiveness = EXCLUDED.offense_passing_plays_explosiveness,
                offense_passing_plays_success_rate = EXCLUDED.offense_passing_plays_success_rate,
                offense_passing_plays_total_ppa = EXCLUDED.offense_passing_plays_total_ppa,
                offense_passing_plays_ppa = EXCLUDED.offense_passing_plays_ppa,
                offense_rushing_plays_explosiveness = EXCLUDED.offense_rushing_plays_explosiveness,
                offense_rushing_plays_success_rate = EXCLUDED.offense_rushing_plays_success_rate,
                offense_rushing_plays_total_ppa = EXCLUDED.offense_rushing_plays_total_ppa,
                offense_rushing_plays_ppa = EXCLUDED.offense_rushing_plays_ppa,
                offense_passing_downs_explosiveness = EXCLUDED.offense_passing_downs_explosiveness,
                offense_passing_downs_success_rate = EXCLUDED.offense_passing_downs_success_rate,
                offense_passing_downs_ppa = EXCLUDED.offense_passing_downs_ppa,
                offense_standard_downs_explosiveness = EXCLUDED.offense_standard_downs_explosiveness,
                offense_standard_downs_success_rate = EXCLUDED.offense_standard_downs_success_rate,
                offense_standard_downs_ppa = EXCLUDED.offense_standard_downs_ppa,
                offense_open_field_yards_total = EXCLUDED.offense_open_field_yards_total,
                offense_open_field_yards = EXCLUDED.offense_open_field_yards,
                offense_second_level_yards_total = EXCLUDED.offense_second_level_yards_total,
                offense_second_level_yards = EXCLUDED.offense_second_level_yards,
                offense_line_yards_total = EXCLUDED.offense_line_yards_total,
                offense_line_yards = EXCLUDED.offense_line_yards,
                offense_stuff_rate = EXCLUDED.offense_stuff_rate,
                offense_power_success = EXCLUDED.offense_power_success,
                offense_explosiveness = EXCLUDED.offense_explosiveness,
                offense_success_rate = EXCLUDED.offense_success_rate,
                offense_total_ppa = EXCLUDED.offense_total_ppa,
                offense_ppa = EXCLUDED.offense_ppa,
                offense_drives = EXCLUDED.offense_drives,
                offense_plays = EXCLUDED.offense_plays,
                defense_passing_plays_explosiveness = EXCLUDED.defense_passing_plays_explosiveness,
                defense_passing_plays_success_rate = EXCLUDED.defense_passing_plays_success_rate,
                defense_passing_plays_total_ppa = EXCLUDED.defense_passing_plays_total_ppa,
                defense_passing_plays_ppa = EXCLUDED.defense_passing_plays_ppa,
                defense_rushing_plays_explosiveness = EXCLUDED.defense_rushing_plays_explosiveness,
                defense_rushing_plays_success_rate = EXCLUDED.defense_rushing_plays_success_rate,
                defense_rushing_plays_total_ppa = EXCLUDED.defense_rushing_plays_total_ppa,
                defense_rushing_plays_ppa = EXCLUDED.defense_rushing_plays_ppa,
                defense_passing_downs_explosiveness = EXCLUDED.defense_passing_downs_explosiveness,
                defense_passing_downs_success_rate = EXCLUDED.defense_passing_downs_success_rate,
                defense_passing_downs_ppa = EXCLUDED.defense_passing_downs_ppa,
                defense_standard_downs_explosiveness = EXCLUDED.defense_standard_downs_explosiveness,
                defense_standard_downs_success_rate = EXCLUDED.defense_standard_downs_success_rate,
                defense_standard_downs_ppa = EXCLUDED.defense_standard_downs_ppa,
                defense_open_field_yards_total = EXCLUDED.defense_open_field_yards_total,
                defense_open_field_yards = EXCLUDED.defense_open_field_yards,
                defense_second_level_yards_total = EXCLUDED.defense_second_level_yards_total,
                defense_second_level_yards = EXCLUDED.defense_second_level_yards,
                defense_line_yards_total = EXCLUDED.defense_line_yards_total,
                defense_line_yards = EXCLUDED.defense_line_yards,
                defense_stuff_rate = EXCLUDED.defense_stuff_rate,
                defense_power_success = EXCLUDED.defense_power_success,
                defense_explosiveness = EXCLUDED.defense_explosiveness,
                defense_success_rate = EXCLUDED.defense_success_rate,
                defense_total_ppa = EXCLUDED.defense_total_ppa,
                defense_ppa = EXCLUDED.defense_ppa,
                defense_drives = EXCLUDED.defense_drives,
                defense_plays = EXCLUDED.defense_plays,
                raw = EXCLUDED.raw
            WHERE (
                game_advanced_stats.opponent_id,
                game_advanced_stats.offense_passing_plays_explosiveness,
                game_advanced_stats.offense_passing_plays_success_rate,
                game_advanced_stats.offense_passing_plays_total_ppa,
                game_advanced_stats.offense_passing_plays_ppa,
                game_advanced_stats.offense_rushing_plays_explosiveness,
                game_advanced_stats.offense_rushing_plays_success_rate,
                game_advanced_stats.offense_rushing_plays_total_ppa,
                game_advanced_stats.offense_rushing_plays_ppa,
                game_advanced_stats.offense_passing_downs_explosiveness,
                game_advanced_stats.offense_passing_downs_success_rate,
                game_advanced_stats.offense_passing_downs_ppa,
                game_advanced_stats.offense_standard_downs_explosiveness,
                game_advanced_stats.offense_standard_downs_success_rate,
                game_advanced_stats.offense_standard_downs_ppa,
                game_advanced_stats.offense_open_field_yards_total,
                game_advanced_stats.offense_open_field_yards,
                game_advanced_stats.offense_second_level_yards_total,
                game_advanced_stats.offense_second_level_yards,
                game_advanced_stats.offense_line_yards_total,
                game_advanced_stats.offense_line_yards,
                game_advanced_stats.offense_stuff_rate,
                game_advanced_stats.offense_power_success,
                game_advanced_stats.offense_explosiveness,
                game_advanced_stats.offense_success_rate,
                game_advanced_stats.offense_total_ppa,
                game_advanced_stats.offense_ppa,
                game_advanced_stats.offense_drives,
                game_advanced_stats.offense_plays,
                game_advanced_stats.defense_passing_plays_explosiveness,
                game_advanced_stats.defense_passing_plays_success_rate,
                game_advanced_stats.defense_passing_plays_total_ppa,
                game_advanced_stats.defense_passing_plays_ppa,
                game_advanced_stats.defense_rushing_plays_explosiveness,
                game_advanced_stats.defense_rushing_plays_success_rate,
                game_advanced_stats.defense_rushing_plays_total_ppa,
                game_advanced_stats.defense_rushing_plays_ppa,
                game_advanced_stats.defense_passing_downs_explosiveness,
                game_advanced_stats.defense_passing_downs_success_rate,
                game_advanced_stats.defense_passing_downs_ppa,
                game_advanced_stats.defense_standard_downs_explosiveness,
                game_advanced_stats.defense_standard_downs_success_rate,
                game_advanced_stats.defense_standard_downs_ppa,
                game_advanced_stats.defense_open_field_yards_total,
                game_advanced_stats.defense_open_field_yards,
                game_advanced_stats.defense_second_level_yards_total,
                game_advanced_stats.defense_second_level_yards,
                game_advanced_stats.defense_line_yards_total,
                game_advanced_stats.defense_line_yards,
                game_advanced_stats.defense_stuff_rate,
                game_advanced_stats.defense_power_success,
                game_advanced_stats.defense_explosiveness,
                game_advanced_stats.defense_success_rate,
                game_advanced_stats.defense_total_ppa,
                game_advanced_stats.defense_ppa,
                game_advanced_stats.defense_drives,
                game_advanced_stats.defense_plays,
                game_advanced_stats.raw
            ) IS DISTINCT FROM (
                EXCLUDED.opponent_id,
                EXCLUDED.offense_passing_plays_explosiveness,
                EXCLUDED.offense_passing_plays_success_rate,
                EXCLUDED.offense_passing_plays_total_ppa,
                EXCLUDED.offense_passing_plays_ppa,
                EXCLUDED.offense_rushing_plays_explosiveness,
                EXCLUDED.offense_rushing_plays_success_rate,
                EXCLUDED.offense_rushing_plays_total_ppa,
                EXCLUDED.offense_rushing_plays_ppa,
                EXCLUDED.offense_passing_downs_explosiveness,
                EXCLUDED.offense_passing_downs_success_rate,
                EXCLUDED.offense_passing_downs_ppa,
                EXCLUDED.offense_standard_downs_explosiveness,
                EXCLUDED.offense_standard_downs_success_rate,
                EXCLUDED.offense_standard_downs_ppa,
                EXCLUDED.offense_open_field_yards_total,
                EXCLUDED.offense_open_field_yards,
                EXCLUDED.offense_second_level_yards_total,
                EXCLUDED.offense_second_level_yards,
                EXCLUDED.offense_line_yards_total,
                EXCLUDED.offense_line_yards,
                EXCLUDED.offense_stuff_rate,
                EXCLUDED.offense_power_success,
                EXCLUDED.offense_explosiveness,
                EXCLUDED.offense_success_rate,
                EXCLUDED.offense_total_ppa,
                EXCLUDED.offense_ppa,
                EXCLUDED.offense_drives,
                EXCLUDED.offense_plays,
                EXCLUDED.defense_passing_plays_explosiveness,
                EXCLUDED.defense_passing_plays_success_rate,
                EXCLUDED.defense_passing_plays_total_ppa,
                EXCLUDED.defense_passing_plays_ppa,
                EXCLUDED.defense_rushing_plays_explosiveness,
                EXCLUDED.defense_rushing_plays_success_rate,
                EXCLUDED.defense_rushing_plays_total_ppa,
                EXCLUDED.defense_rushing_plays_ppa,
                EXCLUDED.defense_passing_downs_explosiveness,
                EXCLUDED.defense_passing_downs_success_rate,
                EXCLUDED.defense_passing_downs_ppa,
                EXCLUDED.defense_standard_downs_explosiveness,
                EXCLUDED.defense_standard_downs_success_rate,
                EXCLUDED.defense_standard_downs_ppa,
                EXCLUDED.defense_open_field_yards_total,
                EXCLUDED.defense_open_field_yards,
                EXCLUDED.defense_second_level_yards_total,
                EXCLUDED.defense_second_level_yards,
                EXCLUDED.defense_line_yards_total,
                EXCLUDED.defense_line_yards,
                EXCLUDED.defense_stuff_rate,
                EXCLUDED.defense_power_success,
                EXCLUDED.defense_explosiveness,
                EXCLUDED.defense_success_rate,
                EXCLUDED.defense_total_ppa,
                EXCLUDED.defense_ppa,
                EXCLUDED.defense_drives,
                EXCLUDED.defense_plays,
                EXCLUDED.raw
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GameAdvancedStats {
    pub id: i32,
    pub game_id: i32,
    pub team_id: i32,
    pub opponent_id: i32,

    pub offense_passing_plays_explosiveness: Option<f64>,
    pub offense_passing_plays_success_rate: Option<f64>,
    pub offense_passing_plays_total_ppa: Option<f64>,
    pub offense_passing_plays_ppa: Option<f64>,
    pub offense_rushing_plays_explosiveness: Option<f64>,
    pub offense_rushing_plays_success_rate: Option<f64>,
    pub offense_rushing_plays_total_ppa: Option<f64>,
    pub offense_rushing_plays_ppa: Option<f64>,
    pub offense_passing_downs_explosiveness: Option<f64>,
    pub offense_passing_downs_success_rate: Option<f64>,
    pub offense_passing_downs_ppa: Option<f64>,
    pub offense_standard_downs_explosiveness: Option<f64>,
    pub offense_standard_downs_success_rate: Option<f64>,
    pub offense_standard_downs_ppa: Option<f64>,
    pub offense_open_field_yards_total: Option<f64>,
    pub offense_open_field_yards: Option<f64>,
    pub offense_second_level_yards_total: Option<f64>,
    pub offense_second_level_yards: Option<f64>,
    pub offense_line_yards_total: Option<f64>,
    pub offense_line_yards: Option<f64>,
    pub offense_stuff_rate: Option<f64>,
    pub offense_power_success: Option<f64>,
    pub offense_explosiveness: Option<f64>,
    pub offense_success_rate: Option<f64>,
    pub offense_total_ppa: Option<f64>,
    pub offense_ppa: Option<f64>,
    pub offense_drives: Option<i32>,
    pub offense_plays: Option<i32>,

    pub defense_passing_plays_explosiveness: Option<f64>,
    pub defense_passing_plays_success_rate: Option<f64>,
    pub defense_passing_plays_total_ppa: Option<f64>,
    pub defense_passing_plays_ppa: Option<f64>,
    pub defense_rushing_plays_explosiveness: Option<f64>,
    pub defense_rushing_plays_success_rate: Option<f64>,
    pub defense_rushing_plays_total_ppa: Option<f64>,
    pub defense_rushing_plays_ppa: Option<f64>,
    pub defense_passing_downs_explosiveness: Option<f64>,
    pub defense_passing_downs_success_rate: Option<f64>,
    pub defense_passing_downs_ppa: Option<f64>,
    pub defense_standard_downs_explosiveness: Option<f64>,
    pub defense_standard_downs_success_rate: Option<f64>,
    pub defense_standard_downs_ppa: Option<f64>,
    pub defense_open_field_yards_total: Option<f64>,
    pub defense_open_field_yards: Option<f64>,
    pub defense_second_level_yards_total: Option<f64>,
    pub defense_second_level_yards: Option<f64>,
    pub defense_line_yards_total: Option<f64>,
    pub defense_line_yards: Option<f64>,
    pub defense_stuff_rate: Option<f64>,
    pub defense_power_success: Option<f64>,
    pub defense_explosiveness: Option<f64>,
    pub defense_success_rate: Option<f64>,
    pub defense_total_ppa: Option<f64>,
    pub defense_ppa: Option<f64>,
    pub defense_drives: Option<i32>,
    pub defense_plays: Option<i32>,

    pub raw: Option<serde_json::Value>,
}
