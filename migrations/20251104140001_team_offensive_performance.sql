-- Per-game offensive performance metrics
-- Based on CFBD's pre-calculated advanced stats
CREATE VIEW analytics.team_offensive_performance AS
SELECT
    g.season,
    g.week,
    g.season_type,
    g.id AS game_id,
    t.school AS team,
    opp.school AS opponent,
    t.conference AS team_conference,

    -- Success Rate Component (higher is better)
    gas.offense_success_rate,
    gas.offense_standard_downs_success_rate,
    gas.offense_passing_downs_success_rate,
    gas.offense_passing_plays_success_rate,
    gas.offense_rushing_plays_success_rate,

    -- Explosiveness Component (higher is better)
    gas.offense_explosiveness,
    gas.offense_passing_plays_explosiveness,
    gas.offense_rushing_plays_explosiveness,
    gas.offense_standard_downs_explosiveness,
    gas.offense_passing_downs_explosiveness,

    -- PPA - Predicted Points Added (higher is better)
    gas.offense_ppa,
    gas.offense_total_ppa,
    gas.offense_passing_plays_ppa,
    gas.offense_passing_plays_total_ppa,
    gas.offense_rushing_plays_ppa,
    gas.offense_rushing_plays_total_ppa,
    gas.offense_standard_downs_ppa,
    gas.offense_passing_downs_ppa,

    -- Line Yards & Efficiency (higher is better)
    gas.offense_line_yards,
    gas.offense_line_yards_total,
    gas.offense_second_level_yards,
    gas.offense_second_level_yards_total,
    gas.offense_open_field_yards,
    gas.offense_open_field_yards_total,
    gas.offense_power_success,
    gas.offense_stuff_rate,

    -- Volume metrics
    gas.offense_plays,
    gas.offense_drives

FROM game_advanced_stats gas
JOIN games g ON gas.game_id = g.id
JOIN teams t ON gas.team_id = t.id
JOIN teams opp ON gas.opponent_id = opp.id;
