-- Season-level offensive performance aggregates
-- Used for calculating team strength and opponent adjustments
CREATE VIEW analytics.team_season_offensive_stats AS
SELECT
    season,
    team,
    team_conference,
    COUNT(*) AS games_played,

    -- Success Rates (simple averages across games)
    AVG(offense_success_rate) AS avg_success_rate,
    AVG(offense_standard_downs_success_rate) AS avg_standard_downs_success_rate,
    AVG(offense_passing_downs_success_rate) AS avg_passing_downs_success_rate,
    AVG(offense_passing_plays_success_rate) AS avg_passing_success_rate,
    AVG(offense_rushing_plays_success_rate) AS avg_rushing_success_rate,

    -- Explosiveness (simple averages across games)
    AVG(offense_explosiveness) AS avg_explosiveness,
    AVG(offense_passing_plays_explosiveness) AS avg_passing_explosiveness,
    AVG(offense_rushing_plays_explosiveness) AS avg_rushing_explosiveness,

    -- PPA (weighted by total plays for accuracy)
    SUM(offense_total_ppa) / NULLIF(SUM(offense_plays), 0) AS avg_ppa_per_play,
    SUM(offense_passing_plays_total_ppa) / NULLIF(SUM(offense_plays), 0) AS avg_passing_ppa_per_play,
    SUM(offense_rushing_plays_total_ppa) / NULLIF(SUM(offense_plays), 0) AS avg_rushing_ppa_per_play,

    -- Efficiency metrics (simple averages)
    AVG(offense_stuff_rate) AS avg_stuff_rate,
    AVG(offense_power_success) AS avg_power_success,
    AVG(offense_line_yards) AS avg_line_yards,

    -- Volume totals
    SUM(offense_plays) AS total_plays,
    SUM(offense_drives) AS total_drives

FROM analytics.team_offensive_performance
GROUP BY season, team, team_conference;
