-- Per-game defensive performance metrics
-- Based on CFBD's pre-calculated advanced stats
-- Note: For defense, LOWER values are generally BETTER (allowing less success/explosiveness)
CREATE VIEW analytics.team_defensive_performance AS
SELECT
    g.season,
    g.week,
    g.season_type,
    g.id AS game_id,
    t.school AS team,
    opp.school AS opponent,
    t.conference AS team_conference,

    -- Success Rate Allowed (lower is better for defense)
    gas.defense_success_rate,
    gas.defense_standard_downs_success_rate,
    gas.defense_passing_downs_success_rate,
    gas.defense_passing_plays_success_rate,
    gas.defense_rushing_plays_success_rate,

    -- Explosiveness Allowed (lower is better for defense)
    gas.defense_explosiveness,
    gas.defense_passing_plays_explosiveness,
    gas.defense_rushing_plays_explosiveness,
    gas.defense_standard_downs_explosiveness,
    gas.defense_passing_downs_explosiveness,

    -- PPA Allowed - Predicted Points Added (lower is better for defense)
    gas.defense_ppa,
    gas.defense_total_ppa,
    gas.defense_passing_plays_ppa,
    gas.defense_passing_plays_total_ppa,
    gas.defense_rushing_plays_ppa,
    gas.defense_rushing_plays_total_ppa,
    gas.defense_standard_downs_ppa,
    gas.defense_passing_downs_ppa,

    -- Line Yards & Efficiency Allowed (lower is better for defense)
    gas.defense_line_yards,
    gas.defense_line_yards_total,
    gas.defense_second_level_yards,
    gas.defense_second_level_yards_total,
    gas.defense_open_field_yards,
    gas.defense_open_field_yards_total,
    gas.defense_power_success,
    gas.defense_stuff_rate,  -- Higher is better for defense (more stuffs)

    -- Volume metrics
    gas.defense_plays,
    gas.defense_drives

FROM game_advanced_stats gas
JOIN games g ON gas.game_id = g.id
JOIN teams t ON gas.team_id = t.id
JOIN teams opp ON gas.opponent_id = opp.id;
