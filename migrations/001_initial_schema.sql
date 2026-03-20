-- ============================================
-- Juel Ratings Database Schema
-- SQLite
-- ============================================

-- 1. Teams
CREATE TABLE IF NOT EXISTS teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cfbd_id INTEGER NOT NULL UNIQUE,
    school TEXT NOT NULL,
    mascot TEXT,
    abbreviation TEXT,
    conference TEXT,
    division TEXT,
    classification TEXT,
    color TEXT,
    alternate_color TEXT,
    twitter TEXT,
    city TEXT,
    state TEXT,
    zip TEXT,
    country_code TEXT,
    timezone TEXT,
    latitude REAL,
    longitude REAL,
    elevation TEXT,
    capacity INTEGER,
    construction_year INTEGER,
    grass INTEGER,
    dome INTEGER,
    alternate_names TEXT
);

-- 2. Games
CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cfbd_id INTEGER NOT NULL UNIQUE,
    season INTEGER NOT NULL,
    week INTEGER NOT NULL,
    season_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    neutral_site INTEGER DEFAULT 0,
    home_team TEXT NOT NULL,
    home_points INTEGER,
    away_team TEXT NOT NULL,
    away_points INTEGER
);

CREATE INDEX idx_games_season_week ON games (season, week);
CREATE INDEX idx_games_neutral_site ON games (neutral_site);

-- 3. Drives
CREATE TABLE IF NOT EXISTS drives (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cfbd_id TEXT NOT NULL UNIQUE,
    game_id INTEGER NOT NULL REFERENCES games(id),
    offense_team_id INTEGER NOT NULL REFERENCES teams(id),
    defense_team_id INTEGER NOT NULL REFERENCES teams(id),
    offense_conference TEXT,
    defense_conference TEXT,
    drive_number INTEGER NOT NULL,
    scoring INTEGER,
    start_period INTEGER,
    start_yardline INTEGER,
    start_yards_to_goal INTEGER,
    start_time_minutes INTEGER,
    start_time_seconds INTEGER,
    end_period INTEGER,
    end_yardline INTEGER,
    end_yards_to_goal INTEGER,
    end_time_minutes INTEGER,
    end_time_seconds INTEGER,
    elapsed_minutes INTEGER,
    elapsed_seconds INTEGER,
    plays INTEGER,
    yards INTEGER,
    drive_result TEXT,
    is_home_offense INTEGER,
    start_offense_score INTEGER,
    start_defense_score INTEGER,
    end_offense_score INTEGER,
    end_defense_score INTEGER
);

-- 4. Game Advanced Stats
CREATE TABLE IF NOT EXISTS game_advanced_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL REFERENCES games(id),
    team_id INTEGER NOT NULL REFERENCES teams(id),
    opponent_id INTEGER NOT NULL REFERENCES teams(id),
    offense_passing_plays_explosiveness REAL,
    offense_passing_plays_success_rate REAL,
    offense_passing_plays_total_ppa REAL,
    offense_passing_plays_ppa REAL,
    offense_rushing_plays_explosiveness REAL,
    offense_rushing_plays_success_rate REAL,
    offense_rushing_plays_total_ppa REAL,
    offense_rushing_plays_ppa REAL,
    offense_passing_downs_explosiveness REAL,
    offense_passing_downs_success_rate REAL,
    offense_passing_downs_ppa REAL,
    offense_standard_downs_explosiveness REAL,
    offense_standard_downs_success_rate REAL,
    offense_standard_downs_ppa REAL,
    offense_open_field_yards_total REAL,
    offense_open_field_yards REAL,
    offense_second_level_yards_total REAL,
    offense_second_level_yards REAL,
    offense_line_yards_total REAL,
    offense_line_yards REAL,
    offense_stuff_rate REAL,
    offense_power_success REAL,
    offense_explosiveness REAL,
    offense_success_rate REAL,
    offense_total_ppa REAL,
    offense_ppa REAL,
    offense_drives INTEGER,
    offense_plays INTEGER,
    defense_passing_plays_explosiveness REAL,
    defense_passing_plays_success_rate REAL,
    defense_passing_plays_total_ppa REAL,
    defense_passing_plays_ppa REAL,
    defense_rushing_plays_explosiveness REAL,
    defense_rushing_plays_success_rate REAL,
    defense_rushing_plays_total_ppa REAL,
    defense_rushing_plays_ppa REAL,
    defense_passing_downs_explosiveness REAL,
    defense_passing_downs_success_rate REAL,
    defense_passing_downs_ppa REAL,
    defense_standard_downs_explosiveness REAL,
    defense_standard_downs_success_rate REAL,
    defense_standard_downs_ppa REAL,
    defense_open_field_yards_total REAL,
    defense_open_field_yards REAL,
    defense_second_level_yards_total REAL,
    defense_second_level_yards REAL,
    defense_line_yards_total REAL,
    defense_line_yards REAL,
    defense_stuff_rate REAL,
    defense_power_success REAL,
    defense_explosiveness REAL,
    defense_success_rate REAL,
    defense_total_ppa REAL,
    defense_ppa REAL,
    defense_drives INTEGER,
    defense_plays INTEGER,
    raw TEXT,
    UNIQUE(game_id, team_id)
);

-- 5. Plays
CREATE TABLE IF NOT EXISTS plays (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cfbd_id TEXT NOT NULL UNIQUE,
    cfbd_drive_id TEXT,
    drive_id INTEGER REFERENCES drives(id),
    game_id INTEGER NOT NULL REFERENCES games(id),
    drive_number INTEGER,
    play_number INTEGER,
    offense TEXT,
    offense_team_id INTEGER NOT NULL REFERENCES teams(id),
    offense_conference TEXT,
    offense_score INTEGER,
    defense TEXT,
    defense_team_id INTEGER NOT NULL REFERENCES teams(id),
    defense_conference TEXT,
    defense_score INTEGER,
    home TEXT,
    away TEXT,
    period INTEGER,
    clock_minutes INTEGER,
    clock_seconds INTEGER,
    offense_timeouts INTEGER,
    defense_timeouts INTEGER,
    yardline INTEGER,
    yards_to_goal INTEGER,
    down INTEGER,
    distance INTEGER,
    yards_gained INTEGER,
    scoring INTEGER,
    play_type TEXT,
    play_text TEXT,
    ppa REAL,
    wallclock TEXT
);

CREATE INDEX idx_plays_game_id ON plays (game_id);
CREATE INDEX idx_plays_drive_id ON plays (drive_id);
CREATE INDEX idx_plays_offense_team_id ON plays (offense_team_id);
CREATE INDEX idx_plays_defense_team_id ON plays (defense_team_id);

-- 6. Havoc
CREATE TABLE IF NOT EXISTS havoc (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL REFERENCES games(id),
    team_id INTEGER NOT NULL REFERENCES teams(id),
    season INTEGER NOT NULL,
    season_type TEXT,
    week INTEGER,
    team TEXT NOT NULL,
    conference TEXT,
    opponent TEXT NOT NULL,
    opponent_conference TEXT,
    offense_db_havoc_rate REAL,
    offense_front_seven_havoc_rate REAL,
    offense_havoc_rate REAL,
    offense_db_havoc_events REAL,
    offense_front_seven_havoc_events REAL,
    offense_total_havoc_events REAL,
    offense_total_plays REAL,
    defense_db_havoc_rate REAL,
    defense_front_seven_havoc_rate REAL,
    defense_havoc_rate REAL,
    defense_db_havoc_events REAL,
    defense_front_seven_havoc_events REAL,
    defense_total_havoc_events REAL,
    defense_total_plays REAL,
    UNIQUE(game_id, team_id)
);

CREATE INDEX idx_havoc_game_id ON havoc (game_id);
CREATE INDEX idx_havoc_team_id ON havoc (team_id);
CREATE INDEX idx_havoc_season_week ON havoc (season, week);

-- ============================================
-- Views
-- ============================================

CREATE VIEW team_offensive_performance AS
SELECT
    g.season,
    g.week,
    g.season_type,
    g.id AS game_id,
    t.school AS team,
    opp.school AS opponent,
    t.conference AS team_conference,
    gas.offense_success_rate,
    gas.offense_standard_downs_success_rate,
    gas.offense_passing_downs_success_rate,
    gas.offense_passing_plays_success_rate,
    gas.offense_rushing_plays_success_rate,
    gas.offense_explosiveness,
    gas.offense_passing_plays_explosiveness,
    gas.offense_rushing_plays_explosiveness,
    gas.offense_standard_downs_explosiveness,
    gas.offense_passing_downs_explosiveness,
    gas.offense_ppa,
    gas.offense_total_ppa,
    gas.offense_passing_plays_ppa,
    gas.offense_passing_plays_total_ppa,
    gas.offense_rushing_plays_ppa,
    gas.offense_rushing_plays_total_ppa,
    gas.offense_standard_downs_ppa,
    gas.offense_passing_downs_ppa,
    gas.offense_line_yards,
    gas.offense_line_yards_total,
    gas.offense_second_level_yards,
    gas.offense_second_level_yards_total,
    gas.offense_open_field_yards,
    gas.offense_open_field_yards_total,
    gas.offense_power_success,
    gas.offense_stuff_rate,
    gas.offense_plays,
    gas.offense_drives
FROM game_advanced_stats gas
JOIN games g ON gas.game_id = g.id
JOIN teams t ON gas.team_id = t.id
JOIN teams opp ON gas.opponent_id = opp.id;

CREATE VIEW team_defensive_performance AS
SELECT
    g.season,
    g.week,
    g.season_type,
    g.id AS game_id,
    t.school AS team,
    opp.school AS opponent,
    t.conference AS team_conference,
    gas.defense_success_rate,
    gas.defense_standard_downs_success_rate,
    gas.defense_passing_downs_success_rate,
    gas.defense_passing_plays_success_rate,
    gas.defense_rushing_plays_success_rate,
    gas.defense_explosiveness,
    gas.defense_passing_plays_explosiveness,
    gas.defense_rushing_plays_explosiveness,
    gas.defense_standard_downs_explosiveness,
    gas.defense_passing_downs_explosiveness,
    gas.defense_ppa,
    gas.defense_total_ppa,
    gas.defense_passing_plays_ppa,
    gas.defense_passing_plays_total_ppa,
    gas.defense_rushing_plays_ppa,
    gas.defense_rushing_plays_total_ppa,
    gas.defense_standard_downs_ppa,
    gas.defense_passing_downs_ppa,
    gas.defense_line_yards,
    gas.defense_line_yards_total,
    gas.defense_second_level_yards,
    gas.defense_second_level_yards_total,
    gas.defense_open_field_yards,
    gas.defense_open_field_yards_total,
    gas.defense_power_success,
    gas.defense_stuff_rate,
    gas.defense_plays,
    gas.defense_drives
FROM game_advanced_stats gas
JOIN games g ON gas.game_id = g.id
JOIN teams t ON gas.team_id = t.id
JOIN teams opp ON gas.opponent_id = opp.id;

CREATE VIEW team_season_offensive_stats AS
SELECT
    season,
    team,
    team_conference,
    COUNT(*) AS games_played,
    AVG(offense_success_rate) AS avg_success_rate,
    AVG(offense_standard_downs_success_rate) AS avg_standard_downs_success_rate,
    AVG(offense_passing_downs_success_rate) AS avg_passing_downs_success_rate,
    AVG(offense_passing_plays_success_rate) AS avg_passing_success_rate,
    AVG(offense_rushing_plays_success_rate) AS avg_rushing_success_rate,
    AVG(offense_explosiveness) AS avg_explosiveness,
    AVG(offense_passing_plays_explosiveness) AS avg_passing_explosiveness,
    AVG(offense_rushing_plays_explosiveness) AS avg_rushing_explosiveness,
    SUM(offense_total_ppa) / NULLIF(SUM(offense_plays), 0) AS avg_ppa_per_play,
    SUM(offense_passing_plays_total_ppa) / NULLIF(SUM(offense_plays), 0) AS avg_passing_ppa_per_play,
    SUM(offense_rushing_plays_total_ppa) / NULLIF(SUM(offense_plays), 0) AS avg_rushing_ppa_per_play,
    AVG(offense_stuff_rate) AS avg_stuff_rate,
    AVG(offense_power_success) AS avg_power_success,
    AVG(offense_line_yards) AS avg_line_yards,
    SUM(offense_plays) AS total_plays,
    SUM(offense_drives) AS total_drives
FROM team_offensive_performance
GROUP BY season, team, team_conference;

CREATE VIEW team_season_defensive_stats AS
SELECT
    season,
    team,
    team_conference,
    COUNT(*) AS games_played,
    AVG(defense_success_rate) AS avg_success_rate_allowed,
    AVG(defense_standard_downs_success_rate) AS avg_standard_downs_success_rate_allowed,
    AVG(defense_passing_downs_success_rate) AS avg_passing_downs_success_rate_allowed,
    AVG(defense_passing_plays_success_rate) AS avg_passing_success_rate_allowed,
    AVG(defense_rushing_plays_success_rate) AS avg_rushing_success_rate_allowed,
    AVG(defense_explosiveness) AS avg_explosiveness_allowed,
    AVG(defense_passing_plays_explosiveness) AS avg_passing_explosiveness_allowed,
    AVG(defense_rushing_plays_explosiveness) AS avg_rushing_explosiveness_allowed,
    SUM(defense_total_ppa) / NULLIF(SUM(defense_plays), 0) AS avg_ppa_allowed_per_play,
    SUM(defense_passing_plays_total_ppa) / NULLIF(SUM(defense_plays), 0) AS avg_passing_ppa_allowed_per_play,
    SUM(defense_rushing_plays_total_ppa) / NULLIF(SUM(defense_plays), 0) AS avg_rushing_ppa_allowed_per_play,
    AVG(defense_stuff_rate) AS avg_stuff_rate,
    AVG(defense_power_success) AS avg_power_success_allowed,
    AVG(defense_line_yards) AS avg_line_yards_allowed,
    SUM(defense_plays) AS total_plays,
    SUM(defense_drives) AS total_drives
FROM team_defensive_performance
GROUP BY season, team, team_conference;

CREATE VIEW opponent_adjusted_performance AS
SELECT
    off.season,
    off.week,
    off.game_id,
    off.team,
    off.opponent,
    off.team_conference,
    off.offense_success_rate AS off_success_rate,
    opp_def.avg_success_rate_allowed AS opp_def_avg_success_rate,
    off.offense_success_rate - opp_def.avg_success_rate_allowed AS off_success_rate_vs_avg,
    off.offense_explosiveness AS off_explosiveness,
    opp_def.avg_explosiveness_allowed AS opp_def_avg_explosiveness,
    off.offense_explosiveness - opp_def.avg_explosiveness_allowed AS off_explosiveness_vs_avg,
    off.offense_ppa AS off_ppa,
    opp_def.avg_ppa_allowed_per_play AS opp_def_avg_ppa,
    off.offense_ppa - opp_def.avg_ppa_allowed_per_play AS off_ppa_vs_avg,
    def.defense_success_rate AS def_success_rate_allowed,
    opp_off.avg_success_rate AS opp_off_avg_success_rate,
    def.defense_success_rate - opp_off.avg_success_rate AS def_success_rate_vs_avg,
    def.defense_explosiveness AS def_explosiveness_allowed,
    opp_off.avg_explosiveness AS opp_off_avg_explosiveness,
    def.defense_explosiveness - opp_off.avg_explosiveness AS def_explosiveness_vs_avg,
    def.defense_ppa AS def_ppa_allowed,
    opp_off.avg_ppa_per_play AS opp_off_avg_ppa,
    def.defense_ppa - opp_off.avg_ppa_per_play AS def_ppa_vs_avg
FROM team_offensive_performance off
JOIN team_defensive_performance def ON def.game_id = off.game_id AND def.team = off.team
LEFT JOIN team_season_defensive_stats opp_def ON opp_def.team = off.opponent AND opp_def.season = off.season
LEFT JOIN team_season_offensive_stats opp_off ON opp_off.team = off.opponent AND opp_off.season = off.season;

-- ============================================
-- Enable foreign key enforcement
-- ============================================
PRAGMA foreign_keys = ON;
