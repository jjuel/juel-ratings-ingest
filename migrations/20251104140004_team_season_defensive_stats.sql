-- Season-level defensive performance aggregates
-- Used for calculating defensive strength and opponent adjustments
-- Note: For defense, LOWER values are generally BETTER (except stuff_rate)
CREATE VIEW analytics.team_season_defensive_stats AS
SELECT
    season,
    team,
    team_conference,
    COUNT(*) AS games_played,

    -- Success Rates Allowed (simple averages across games, lower is better)
    AVG(defense_success_rate) AS avg_success_rate_allowed,
    AVG(defense_standard_downs_success_rate) AS avg_standard_downs_success_rate_allowed,
    AVG(defense_passing_downs_success_rate) AS avg_passing_downs_success_rate_allowed,
    AVG(defense_passing_plays_success_rate) AS avg_passing_success_rate_allowed,
    AVG(defense_rushing_plays_success_rate) AS avg_rushing_success_rate_allowed,

    -- Explosiveness Allowed (simple averages across games, lower is better)
    AVG(defense_explosiveness) AS avg_explosiveness_allowed,
    AVG(defense_passing_plays_explosiveness) AS avg_passing_explosiveness_allowed,
    AVG(defense_rushing_plays_explosiveness) AS avg_rushing_explosiveness_allowed,

    -- PPA Allowed (weighted by total plays for accuracy, lower is better)
    SUM(defense_total_ppa) / NULLIF(SUM(defense_plays), 0) AS avg_ppa_allowed_per_play,
    SUM(defense_passing_plays_total_ppa) / NULLIF(SUM(defense_plays), 0) AS avg_passing_ppa_allowed_per_play,
    SUM(defense_rushing_plays_total_ppa) / NULLIF(SUM(defense_plays), 0) AS avg_rushing_ppa_allowed_per_play,

    -- Efficiency metrics (simple averages)
    AVG(defense_stuff_rate) AS avg_stuff_rate,  -- Higher is better for defense
    AVG(defense_power_success) AS avg_power_success_allowed,  -- Lower is better
    AVG(defense_line_yards) AS avg_line_yards_allowed,  -- Lower is better

    -- Volume totals
    SUM(defense_plays) AS total_plays,
    SUM(defense_drives) AS total_drives

FROM analytics.team_defensive_performance
GROUP BY season, team, team_conference;
