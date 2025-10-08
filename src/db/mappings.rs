use crate::cfbd;
use crate::db;
use serde_json::to_value;
use std::collections::HashMap;

impl From<cfbd::Team> for db::Team {
    fn from(api: cfbd::Team) -> Self {
        Self {
            id: 0,
            cfbd_id: api.id,
            school: api.school,
            mascot: api.mascot,
            abbreviation: api.abbreviation,
            conference: api.conference,
            division: api.division,
            classification: api.classification,
            color: api.color,
            alternate_color: api.alternate_color,
            twitter: api.twitter,
            city: api.location.as_ref().and_then(|l| l.city.clone()),
            state: api.location.as_ref().and_then(|l| l.state.clone()),
            zip: api.location.as_ref().and_then(|l| l.zip.clone()),
            country_code: api.location.as_ref().and_then(|l| l.country_code.clone()),
            timezone: api.location.as_ref().and_then(|l| l.timezone.clone()),
            latitude: api.location.as_ref().and_then(|l| l.latitude),
            longitude: api.location.as_ref().and_then(|l| l.longitude),
            elevation: api.location.as_ref().and_then(|l| l.elevation.clone()),
            capacity: api.location.as_ref().and_then(|l| l.capacity),
            construction_year: api.location.as_ref().and_then(|l| l.construction_year),
            grass: api.location.as_ref().and_then(|l| l.grass),
            dome: api.location.as_ref().and_then(|l| l.dome),
            alternate_names: api.alternate_names,
        }
    }
}

impl From<(&cfbd::Drive, i32, i32, i32)> for db::Drive {
    fn from((api, game_id, offense_team_id, defense_team_id): (&cfbd::Drive, i32, i32, i32)) -> Self {
        let (start_time_minutes, start_time_seconds) = split_clock(api.start_time.as_ref());
        let (end_time_minutes, end_time_seconds) = split_clock(api.end_time.as_ref());
        let (elapsed_minutes, elapsed_seconds) = split_clock(api.elapsed.as_ref());

        db::Drive {
            id: 0,
            cfbd_id: api.id.clone(),
            game_id,
            offense_team_id,
            defense_team_id,
            offense_conference: api.offense_conference.clone(),
            defense_conference: api.defense_conference.clone(),
            drive_number: api.drive_number,
            scoring: api.scoring,
            start_period: api.start_period,
            start_yardline: api.start_yardline,
            start_yards_to_goal: api.start_yards_to_goal,
            start_time_minutes,
            start_time_seconds,
            end_period: api.end_period,
            end_yardline: api.end_yardline,
            end_yards_to_goal: api.end_yards_to_goal,
            end_time_minutes,
            end_time_seconds,
            elapsed_minutes,
            elapsed_seconds,
            plays: api.plays,
            yards: api.yards,
            drive_result: api.drive_result.clone(),
            is_home_offense: api.is_home_offense,
            start_offense_score: api.start_offense_score,
            start_defense_score: api.start_defense_score,
            end_offense_score: api.end_offense_score,
            end_defense_score: api.end_defense_score,
        }
    }
}

pub fn map_drive(
    api: &cfbd::Drive,
    game_id: i32,
    teams_by_name: &HashMap<String, i32>,
) -> Option<db::Drive> {
    let offense_team_id = *teams_by_name.get(&api.offense)?;
    let defense_team_id = *teams_by_name.get(&api.defense)?;

    Some(db::Drive::from((api, game_id, offense_team_id, defense_team_id)))
}

fn split_clock(clock: Option<&cfbd::drives::DriveClock>) -> (Option<i32>, Option<i32>) {
    match clock {
        Some(value) => (value.minutes, value.seconds),
        None => (None, None),
    }
}

impl From<(i32, i32, i32, &cfbd::GameAdvancedStats)> for db::GameAdvancedStats {
    fn from(
        (game_id, team_id, opponent_id, api): (i32, i32, i32, &cfbd::GameAdvancedStats),
    ) -> Self {
        let offense = &api.offense;
        let defense = &api.defense;

        db::GameAdvancedStats {
            id: 0,
            game_id,
            team_id,
            opponent_id,

            offense_passing_plays_explosiveness: offense.passing_plays.explosiveness,
            offense_passing_plays_success_rate: offense.passing_plays.success_rate,
            offense_passing_plays_total_ppa: offense.passing_plays.total_ppa,
            offense_passing_plays_ppa: offense.passing_plays.ppa,
            offense_rushing_plays_explosiveness: offense.rushing_plays.explosiveness,
            offense_rushing_plays_success_rate: offense.rushing_plays.success_rate,
            offense_rushing_plays_total_ppa: offense.rushing_plays.total_ppa,
            offense_rushing_plays_ppa: offense.rushing_plays.ppa,
            offense_passing_downs_explosiveness: offense.passing_downs.explosiveness,
            offense_passing_downs_success_rate: offense.passing_downs.success_rate,
            offense_passing_downs_ppa: offense.passing_downs.ppa,
            offense_standard_downs_explosiveness: offense.standard_downs.explosiveness,
            offense_standard_downs_success_rate: offense.standard_downs.success_rate,
            offense_standard_downs_ppa: offense.standard_downs.ppa,
            offense_open_field_yards_total: offense.open_field_yards_total,
            offense_open_field_yards: offense.open_field_yards,
            offense_second_level_yards_total: offense.second_level_yards_total,
            offense_second_level_yards: offense.second_level_yards,
            offense_line_yards_total: offense.line_yards_total,
            offense_line_yards: offense.line_yards,
            offense_stuff_rate: offense.stuff_rate,
            offense_power_success: offense.power_success,
            offense_explosiveness: offense.explosiveness,
            offense_success_rate: offense.success_rate,
            offense_total_ppa: offense.total_ppa,
            offense_ppa: offense.ppa,
            offense_drives: offense.drives,
            offense_plays: offense.plays,

            defense_passing_plays_explosiveness: defense.passing_plays.explosiveness,
            defense_passing_plays_success_rate: defense.passing_plays.success_rate,
            defense_passing_plays_total_ppa: defense.passing_plays.total_ppa,
            defense_passing_plays_ppa: defense.passing_plays.ppa,
            defense_rushing_plays_explosiveness: defense.rushing_plays.explosiveness,
            defense_rushing_plays_success_rate: defense.rushing_plays.success_rate,
            defense_rushing_plays_total_ppa: defense.rushing_plays.total_ppa,
            defense_rushing_plays_ppa: defense.rushing_plays.ppa,
            defense_passing_downs_explosiveness: defense.passing_downs.explosiveness,
            defense_passing_downs_success_rate: defense.passing_downs.success_rate,
            defense_passing_downs_ppa: defense.passing_downs.ppa,
            defense_standard_downs_explosiveness: defense.standard_downs.explosiveness,
            defense_standard_downs_success_rate: defense.standard_downs.success_rate,
            defense_standard_downs_ppa: defense.standard_downs.ppa,
            defense_open_field_yards_total: defense.open_field_yards_total,
            defense_open_field_yards: defense.open_field_yards,
            defense_second_level_yards_total: defense.second_level_yards_total,
            defense_second_level_yards: defense.second_level_yards,
            defense_line_yards_total: defense.line_yards_total,
            defense_line_yards: defense.line_yards,
            defense_stuff_rate: defense.stuff_rate,
            defense_power_success: defense.power_success,
            defense_explosiveness: defense.explosiveness,
            defense_success_rate: defense.success_rate,
            defense_total_ppa: defense.total_ppa,
            defense_ppa: defense.ppa,
            defense_drives: defense.drives,
            defense_plays: defense.plays,

            raw: Some(to_value(api).unwrap_or_default()),
        }
    }
}
