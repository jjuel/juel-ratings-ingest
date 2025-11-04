-- Opponent-adjusted performance metrics
-- Compares team's game performance vs opponent's season average
-- Used for strength of schedule adjustments in ratings calculations
CREATE VIEW analytics.opponent_adjusted_performance AS
SELECT
    off.season,
    off.week,
    off.game_id,
    off.team,
    off.opponent,
    off.team_conference,

    -- Offensive Performance vs Opponent's Defensive Strength
    -- Positive means team performed better than opponent typically allows
    off.offense_success_rate AS off_success_rate,
    opp_def.avg_success_rate_allowed AS opp_def_avg_success_rate,
    off.offense_success_rate - opp_def.avg_success_rate_allowed AS off_success_rate_vs_avg,

    off.offense_explosiveness AS off_explosiveness,
    opp_def.avg_explosiveness_allowed AS opp_def_avg_explosiveness,
    off.offense_explosiveness - opp_def.avg_explosiveness_allowed AS off_explosiveness_vs_avg,

    off.offense_ppa AS off_ppa,
    opp_def.avg_ppa_allowed_per_play AS opp_def_avg_ppa,
    off.offense_ppa - opp_def.avg_ppa_allowed_per_play AS off_ppa_vs_avg,

    -- Defensive Performance vs Opponent's Offensive Strength
    -- Negative means team held opponent below their typical performance (good for defense)
    def.defense_success_rate AS def_success_rate_allowed,
    opp_off.avg_success_rate AS opp_off_avg_success_rate,
    def.defense_success_rate - opp_off.avg_success_rate AS def_success_rate_vs_avg,

    def.defense_explosiveness AS def_explosiveness_allowed,
    opp_off.avg_explosiveness AS opp_off_avg_explosiveness,
    def.defense_explosiveness - opp_off.avg_explosiveness AS def_explosiveness_vs_avg,

    def.defense_ppa AS def_ppa_allowed,
    opp_off.avg_ppa_per_play AS opp_off_avg_ppa,
    def.defense_ppa - opp_off.avg_ppa_per_play AS def_ppa_vs_avg

FROM analytics.team_offensive_performance off
JOIN analytics.team_defensive_performance def
    ON def.game_id = off.game_id
    AND def.team = off.team
LEFT JOIN analytics.team_season_defensive_stats opp_def
    ON opp_def.team = off.opponent
    AND opp_def.season = off.season
LEFT JOIN analytics.team_season_offensive_stats opp_off
    ON opp_off.team = off.opponent
    AND opp_off.season = off.season;
