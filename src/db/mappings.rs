use crate::db;
use chrono::{DateTime, Utc};
use serde_json::to_value;
use std::collections::HashMap;

use crate::api::drives::{Drive as ApiDrive, DriveClock};
use crate::api::game_advanced_stats::GameAdvancedStats as ApiGameAdvancedStats;
use crate::api::games::Game as ApiGame;
use crate::api::havoc::Havoc as ApiHavoc;
use crate::api::plays::Play as ApiPlay;
use crate::api::teams::Team as ApiTeam;

pub fn map_team(api: &ApiTeam) -> db::Team {
    db::Team {
        id: 0,
        cfbd_id: api.id,
        school: api.school.clone(),
        mascot: api.mascot.clone(),
        abbreviation: api.abbreviation.clone(),
        conference: api.conference.clone(),
        division: api.division.clone(),
        classification: api.classification.clone(),
        color: api.color.clone(),
        alternate_color: api.alternate_color.clone(),
        twitter: api.twitter.clone(),
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
        alternate_names: api.alternate_names.clone(),
    }
}

pub fn map_game(api: &ApiGame) -> db::Game {
    let start_date = api
        .start_date
        .parse::<DateTime<Utc>>()
        .expect("Failed to parse start_date");

    db::Game {
        id: 0,
        cfbd_id: api.id,
        season: api.season,
        week: api.week,
        season_type: api.season_type.clone(),
        start_date,
        neutral_site: api.neutral_site,
        home_team: api.home_team.clone(),
        home_points: api.home_points,
        away_team: api.away_team.clone(),
        away_points: api.away_points,
    }
}

pub fn map_drive(
    api: &ApiDrive,
    game_id: i32,
    teams_by_name: &HashMap<String, i32>,
) -> Option<db::Drive> {
    let offense_team_id = *teams_by_name.get(&api.offense)?;
    let defense_team_id = *teams_by_name.get(&api.defense)?;

    let (start_time_minutes, start_time_seconds) = split_clock(api.start_time.as_ref());
    let (end_time_minutes, end_time_seconds) = split_clock(api.end_time.as_ref());
    let (elapsed_minutes, elapsed_seconds) = split_clock(api.elapsed.as_ref());

    Some(db::Drive {
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
    })
}

fn split_clock(clock: Option<&DriveClock>) -> (Option<i32>, Option<i32>) {
    match clock {
        Some(value) => (value.minutes, value.seconds),
        None => (None, None),
    }
}

pub fn map_game_advanced_stats(
    api: &ApiGameAdvancedStats,
    game_id: i32,
    teams_by_name: &HashMap<String, i32>,
) -> Option<db::GameAdvancedStats> {
    let team_id = *teams_by_name.get(&api.team)?;
    let opponent_id = *teams_by_name.get(&api.opponent)?;

    let offense = &api.offense;
    let defense = &api.defense;

    Some(db::GameAdvancedStats {
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
    })
}

pub fn map_play(
    api: &ApiPlay,
    game_id: i32,
    teams_by_name: &HashMap<String, i32>,
    drives_by_cfbd_id: &HashMap<String, i32>,
) -> Option<db::Play> {
    // Both offense and defense team names are required
    let offense_team_id = api.offense.as_ref().and_then(|name| teams_by_name.get(name))?;
    let defense_team_id = api.defense.as_ref().and_then(|name| teams_by_name.get(name))?;

    // Split clock into minutes and seconds
    let (clock_minutes, clock_seconds) = match &api.clock {
        Some(clock) => (Some(clock.minutes), Some(clock.seconds)),
        None => (None, None),
    };

    // Look up drive_id by cfbd_drive_id if available
    let drive_id = api.drive_id.as_ref().and_then(|cfbd_drive_id| drives_by_cfbd_id.get(cfbd_drive_id)).copied();

    // Require cfbd_id
    let cfbd_id = api.id.as_ref()?.clone();

    Some(db::Play {
        id: 0,
        cfbd_id,
        cfbd_drive_id: api.drive_id.clone(),
        drive_id,
        game_id,
        drive_number: api.drive_number,
        play_number: api.play_number,
        offense: api.offense.clone(),
        offense_team_id: *offense_team_id,
        offense_conference: api.offense_conference.clone(),
        offense_score: api.offense_score,
        defense: api.defense.clone(),
        defense_team_id: *defense_team_id,
        defense_conference: api.defense_conference.clone(),
        defense_score: api.defense_score,
        home: api.home.clone(),
        away: api.away.clone(),
        period: api.period,
        clock_minutes,
        clock_seconds,
        offense_timeouts: api.offense_timeouts,
        defense_timeouts: api.defense_timeouts,
        yardline: api.yardline,
        yards_to_goal: api.yards_to_goal,
        down: api.down,
        distance: api.distance,
        yards_gained: api.yards_gained,
        scoring: api.scoring,
        play_type: api.play_type.clone(),
        play_text: api.play_text.clone(),
        ppa: api.ppa,
        wallclock: api.wallclock.clone(),
    })
}

pub fn map_havoc(
    api: &ApiHavoc,
    game_id: i32,
    teams_by_name: &HashMap<String, i32>,
) -> Option<db::Havoc> {
    // Team name is required for lookup
    let team_id = teams_by_name.get(&api.team)?;

    // Extract offense stats
    let (
        offense_db_havoc_rate,
        offense_front_seven_havoc_rate,
        offense_havoc_rate,
        offense_db_havoc_events,
        offense_front_seven_havoc_events,
        offense_total_havoc_events,
        offense_total_plays,
    ) = match &api.offense {
        Some(offense) => (
            offense.db_havoc_rate,
            offense.front_seven_havoc_rate,
            offense.havoc_rate,
            offense.db_havoc_events,
            offense.front_seven_havoc_events,
            offense.total_havoc_events,
            offense.total_plays,
        ),
        None => (None, None, None, None, None, None, None),
    };

    // Extract defense stats
    let (
        defense_db_havoc_rate,
        defense_front_seven_havoc_rate,
        defense_havoc_rate,
        defense_db_havoc_events,
        defense_front_seven_havoc_events,
        defense_total_havoc_events,
        defense_total_plays,
    ) = match &api.defense {
        Some(defense) => (
            defense.db_havoc_rate,
            defense.front_seven_havoc_rate,
            defense.havoc_rate,
            defense.db_havoc_events,
            defense.front_seven_havoc_events,
            defense.total_havoc_events,
            defense.total_plays,
        ),
        None => (None, None, None, None, None, None, None),
    };

    Some(db::Havoc {
        id: 0,
        game_id,
        team_id: *team_id,
        season: api.season,
        season_type: api.season_type.clone(),
        week: api.week,
        team: api.team.clone(),
        conference: api.conference.clone(),
        opponent: api.opponent.clone(),
        opponent_conference: api.opponent_conference.clone(),
        offense_db_havoc_rate,
        offense_front_seven_havoc_rate,
        offense_havoc_rate,
        offense_db_havoc_events,
        offense_front_seven_havoc_events,
        offense_total_havoc_events,
        offense_total_plays,
        defense_db_havoc_rate,
        defense_front_seven_havoc_rate,
        defense_havoc_rate,
        defense_db_havoc_events,
        defense_front_seven_havoc_events,
        defense_total_havoc_events,
        defense_total_plays,
    })
}
