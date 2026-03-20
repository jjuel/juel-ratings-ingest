use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

pub struct UpsertStats {
    pub ids: Vec<i32>,
    pub inserted: usize,
    pub updated: usize,
}

pub async fn upsert_batch(
    pool: &SqlitePool,
    stats: &[GameAdvancedStats],
) -> Result<UpsertStats, sqlx::Error> {
    if stats.is_empty() {
        return Ok(UpsertStats {
            ids: vec![],
            inserted: 0,
            updated: 0,
        });
    }

    let mut tx = pool.begin().await?;

    let mut existing_set = std::collections::HashSet::new();
    for stat in stats {
        let exists: Option<(i32,)> =
            sqlx::query_as("SELECT 1 FROM game_advanced_stats WHERE game_id = ? AND team_id = ?")
                .bind(stat.game_id)
                .bind(stat.team_id)
                .fetch_optional(&mut *tx)
                .await?;

        if exists.is_some() {
            existing_set.insert((stat.game_id, stat.team_id));
        }
    }

    let mut all_ids = Vec::new();
    let mut total_inserted = 0;
    let mut total_updated = 0;

    for stat in stats {
        if existing_set.contains(&(stat.game_id, stat.team_id)) {
            total_updated += 1;
        } else {
            total_inserted += 1;
        }

        let placeholders = std::iter::repeat_n("?", 60).collect::<Vec<_>>().join(", ");
        let sql = format!(
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
            ) VALUES ({})
            ON CONFLICT(game_id, team_id) DO UPDATE SET
                opponent_id = excluded.opponent_id,
                offense_passing_plays_explosiveness = excluded.offense_passing_plays_explosiveness,
                offense_passing_plays_success_rate = excluded.offense_passing_plays_success_rate,
                offense_passing_plays_total_ppa = excluded.offense_passing_plays_total_ppa,
                offense_passing_plays_ppa = excluded.offense_passing_plays_ppa,
                offense_rushing_plays_explosiveness = excluded.offense_rushing_plays_explosiveness,
                offense_rushing_plays_success_rate = excluded.offense_rushing_plays_success_rate,
                offense_rushing_plays_total_ppa = excluded.offense_rushing_plays_total_ppa,
                offense_rushing_plays_ppa = excluded.offense_rushing_plays_ppa,
                offense_passing_downs_explosiveness = excluded.offense_passing_downs_explosiveness,
                offense_passing_downs_success_rate = excluded.offense_passing_downs_success_rate,
                offense_passing_downs_ppa = excluded.offense_passing_downs_ppa,
                offense_standard_downs_explosiveness = excluded.offense_standard_downs_explosiveness,
                offense_standard_downs_success_rate = excluded.offense_standard_downs_success_rate,
                offense_standard_downs_ppa = excluded.offense_standard_downs_ppa,
                offense_open_field_yards_total = excluded.offense_open_field_yards_total,
                offense_open_field_yards = excluded.offense_open_field_yards,
                offense_second_level_yards_total = excluded.offense_second_level_yards_total,
                offense_second_level_yards = excluded.offense_second_level_yards,
                offense_line_yards_total = excluded.offense_line_yards_total,
                offense_line_yards = excluded.offense_line_yards,
                offense_stuff_rate = excluded.offense_stuff_rate,
                offense_power_success = excluded.offense_power_success,
                offense_explosiveness = excluded.offense_explosiveness,
                offense_success_rate = excluded.offense_success_rate,
                offense_total_ppa = excluded.offense_total_ppa,
                offense_ppa = excluded.offense_ppa,
                offense_drives = excluded.offense_drives,
                offense_plays = excluded.offense_plays,
                defense_passing_plays_explosiveness = excluded.defense_passing_plays_explosiveness,
                defense_passing_plays_success_rate = excluded.defense_passing_plays_success_rate,
                defense_passing_plays_total_ppa = excluded.defense_passing_plays_total_ppa,
                defense_passing_plays_ppa = excluded.defense_passing_plays_ppa,
                defense_rushing_plays_explosiveness = excluded.defense_rushing_plays_explosiveness,
                defense_rushing_plays_success_rate = excluded.defense_rushing_plays_success_rate,
                defense_rushing_plays_total_ppa = excluded.defense_rushing_plays_total_ppa,
                defense_rushing_plays_ppa = excluded.defense_rushing_plays_ppa,
                defense_passing_downs_explosiveness = excluded.defense_passing_downs_explosiveness,
                defense_passing_downs_success_rate = excluded.defense_passing_downs_success_rate,
                defense_passing_downs_ppa = excluded.defense_passing_downs_ppa,
                defense_standard_downs_explosiveness = excluded.defense_standard_downs_explosiveness,
                defense_standard_downs_success_rate = excluded.defense_standard_downs_success_rate,
                defense_standard_downs_ppa = excluded.defense_standard_downs_ppa,
                defense_open_field_yards_total = excluded.defense_open_field_yards_total,
                defense_open_field_yards = excluded.defense_open_field_yards,
                defense_second_level_yards_total = excluded.defense_second_level_yards_total,
                defense_second_level_yards = excluded.defense_second_level_yards,
                defense_line_yards_total = excluded.defense_line_yards_total,
                defense_line_yards = excluded.defense_line_yards,
                defense_stuff_rate = excluded.defense_stuff_rate,
                defense_power_success = excluded.defense_power_success,
                defense_explosiveness = excluded.defense_explosiveness,
                defense_success_rate = excluded.defense_success_rate,
                defense_total_ppa = excluded.defense_total_ppa,
                defense_ppa = excluded.defense_ppa,
                defense_drives = excluded.defense_drives,
                defense_plays = excluded.defense_plays,
                raw = excluded.raw
            WHERE game_advanced_stats.opponent_id IS NOT excluded.opponent_id
                OR game_advanced_stats.offense_passing_plays_explosiveness IS NOT excluded.offense_passing_plays_explosiveness
                OR game_advanced_stats.offense_passing_plays_success_rate IS NOT excluded.offense_passing_plays_success_rate
                OR game_advanced_stats.offense_passing_plays_total_ppa IS NOT excluded.offense_passing_plays_total_ppa
                OR game_advanced_stats.offense_passing_plays_ppa IS NOT excluded.offense_passing_plays_ppa
                OR game_advanced_stats.offense_rushing_plays_explosiveness IS NOT excluded.offense_rushing_plays_explosiveness
                OR game_advanced_stats.offense_rushing_plays_success_rate IS NOT excluded.offense_rushing_plays_success_rate
                OR game_advanced_stats.offense_rushing_plays_total_ppa IS NOT excluded.offense_rushing_plays_total_ppa
                OR game_advanced_stats.offense_rushing_plays_ppa IS NOT excluded.offense_rushing_plays_ppa
                OR game_advanced_stats.offense_passing_downs_explosiveness IS NOT excluded.offense_passing_downs_explosiveness
                OR game_advanced_stats.offense_passing_downs_success_rate IS NOT excluded.offense_passing_downs_success_rate
                OR game_advanced_stats.offense_passing_downs_ppa IS NOT excluded.offense_passing_downs_ppa
                OR game_advanced_stats.offense_standard_downs_explosiveness IS NOT excluded.offense_standard_downs_explosiveness
                OR game_advanced_stats.offense_standard_downs_success_rate IS NOT excluded.offense_standard_downs_success_rate
                OR game_advanced_stats.offense_standard_downs_ppa IS NOT excluded.offense_standard_downs_ppa
                OR game_advanced_stats.offense_open_field_yards_total IS NOT excluded.offense_open_field_yards_total
                OR game_advanced_stats.offense_open_field_yards IS NOT excluded.offense_open_field_yards
                OR game_advanced_stats.offense_second_level_yards_total IS NOT excluded.offense_second_level_yards_total
                OR game_advanced_stats.offense_second_level_yards IS NOT excluded.offense_second_level_yards
                OR game_advanced_stats.offense_line_yards_total IS NOT excluded.offense_line_yards_total
                OR game_advanced_stats.offense_line_yards IS NOT excluded.offense_line_yards
                OR game_advanced_stats.offense_stuff_rate IS NOT excluded.offense_stuff_rate
                OR game_advanced_stats.offense_power_success IS NOT excluded.offense_power_success
                OR game_advanced_stats.offense_explosiveness IS NOT excluded.offense_explosiveness
                OR game_advanced_stats.offense_success_rate IS NOT excluded.offense_success_rate
                OR game_advanced_stats.offense_total_ppa IS NOT excluded.offense_total_ppa
                OR game_advanced_stats.offense_ppa IS NOT excluded.offense_ppa
                OR game_advanced_stats.offense_drives IS NOT excluded.offense_drives
                OR game_advanced_stats.offense_plays IS NOT excluded.offense_plays
                OR game_advanced_stats.defense_passing_plays_explosiveness IS NOT excluded.defense_passing_plays_explosiveness
                OR game_advanced_stats.defense_passing_plays_success_rate IS NOT excluded.defense_passing_plays_success_rate
                OR game_advanced_stats.defense_passing_plays_total_ppa IS NOT excluded.defense_passing_plays_total_ppa
                OR game_advanced_stats.defense_passing_plays_ppa IS NOT excluded.defense_passing_plays_ppa
                OR game_advanced_stats.defense_rushing_plays_explosiveness IS NOT excluded.defense_rushing_plays_explosiveness
                OR game_advanced_stats.defense_rushing_plays_success_rate IS NOT excluded.defense_rushing_plays_success_rate
                OR game_advanced_stats.defense_rushing_plays_total_ppa IS NOT excluded.defense_rushing_plays_total_ppa
                OR game_advanced_stats.defense_rushing_plays_ppa IS NOT excluded.defense_rushing_plays_ppa
                OR game_advanced_stats.defense_passing_downs_explosiveness IS NOT excluded.defense_passing_downs_explosiveness
                OR game_advanced_stats.defense_passing_downs_success_rate IS NOT excluded.defense_passing_downs_success_rate
                OR game_advanced_stats.defense_passing_downs_ppa IS NOT excluded.defense_passing_downs_ppa
                OR game_advanced_stats.defense_standard_downs_explosiveness IS NOT excluded.defense_standard_downs_explosiveness
                OR game_advanced_stats.defense_standard_downs_success_rate IS NOT excluded.defense_standard_downs_success_rate
                OR game_advanced_stats.defense_standard_downs_ppa IS NOT excluded.defense_standard_downs_ppa
                OR game_advanced_stats.defense_open_field_yards_total IS NOT excluded.defense_open_field_yards_total
                OR game_advanced_stats.defense_open_field_yards IS NOT excluded.defense_open_field_yards
                OR game_advanced_stats.defense_second_level_yards_total IS NOT excluded.defense_second_level_yards_total
                OR game_advanced_stats.defense_second_level_yards IS NOT excluded.defense_second_level_yards
                OR game_advanced_stats.defense_line_yards_total IS NOT excluded.defense_line_yards_total
                OR game_advanced_stats.defense_line_yards IS NOT excluded.defense_line_yards
                OR game_advanced_stats.defense_stuff_rate IS NOT excluded.defense_stuff_rate
                OR game_advanced_stats.defense_power_success IS NOT excluded.defense_power_success
                OR game_advanced_stats.defense_explosiveness IS NOT excluded.defense_explosiveness
                OR game_advanced_stats.defense_success_rate IS NOT excluded.defense_success_rate
                OR game_advanced_stats.defense_total_ppa IS NOT excluded.defense_total_ppa
                OR game_advanced_stats.defense_ppa IS NOT excluded.defense_ppa
                OR game_advanced_stats.defense_drives IS NOT excluded.defense_drives
                OR game_advanced_stats.defense_plays IS NOT excluded.defense_plays
                OR game_advanced_stats.raw IS NOT excluded.raw
            RETURNING id",
            placeholders
        );

        let id = sqlx::query_as::<_, (i32,)>(&sql)
            .bind(stat.game_id)
            .bind(stat.team_id)
            .bind(stat.opponent_id)
            .bind(stat.offense_passing_plays_explosiveness)
            .bind(stat.offense_passing_plays_success_rate)
            .bind(stat.offense_passing_plays_total_ppa)
            .bind(stat.offense_passing_plays_ppa)
            .bind(stat.offense_rushing_plays_explosiveness)
            .bind(stat.offense_rushing_plays_success_rate)
            .bind(stat.offense_rushing_plays_total_ppa)
            .bind(stat.offense_rushing_plays_ppa)
            .bind(stat.offense_passing_downs_explosiveness)
            .bind(stat.offense_passing_downs_success_rate)
            .bind(stat.offense_passing_downs_ppa)
            .bind(stat.offense_standard_downs_explosiveness)
            .bind(stat.offense_standard_downs_success_rate)
            .bind(stat.offense_standard_downs_ppa)
            .bind(stat.offense_open_field_yards_total)
            .bind(stat.offense_open_field_yards)
            .bind(stat.offense_second_level_yards_total)
            .bind(stat.offense_second_level_yards)
            .bind(stat.offense_line_yards_total)
            .bind(stat.offense_line_yards)
            .bind(stat.offense_stuff_rate)
            .bind(stat.offense_power_success)
            .bind(stat.offense_explosiveness)
            .bind(stat.offense_success_rate)
            .bind(stat.offense_total_ppa)
            .bind(stat.offense_ppa)
            .bind(stat.offense_drives)
            .bind(stat.offense_plays)
            .bind(stat.defense_passing_plays_explosiveness)
            .bind(stat.defense_passing_plays_success_rate)
            .bind(stat.defense_passing_plays_total_ppa)
            .bind(stat.defense_passing_plays_ppa)
            .bind(stat.defense_rushing_plays_explosiveness)
            .bind(stat.defense_rushing_plays_success_rate)
            .bind(stat.defense_rushing_plays_total_ppa)
            .bind(stat.defense_rushing_plays_ppa)
            .bind(stat.defense_passing_downs_explosiveness)
            .bind(stat.defense_passing_downs_success_rate)
            .bind(stat.defense_passing_downs_ppa)
            .bind(stat.defense_standard_downs_explosiveness)
            .bind(stat.defense_standard_downs_success_rate)
            .bind(stat.defense_standard_downs_ppa)
            .bind(stat.defense_open_field_yards_total)
            .bind(stat.defense_open_field_yards)
            .bind(stat.defense_second_level_yards_total)
            .bind(stat.defense_second_level_yards)
            .bind(stat.defense_line_yards_total)
            .bind(stat.defense_line_yards)
            .bind(stat.defense_stuff_rate)
            .bind(stat.defense_power_success)
            .bind(stat.defense_explosiveness)
            .bind(stat.defense_success_rate)
            .bind(stat.defense_total_ppa)
            .bind(stat.defense_ppa)
            .bind(stat.defense_drives)
            .bind(stat.defense_plays)
            .bind(&stat.raw)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(id) = id {
            all_ids.push(id.0);
        } else if existing_set.contains(&(stat.game_id, stat.team_id)) {
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
