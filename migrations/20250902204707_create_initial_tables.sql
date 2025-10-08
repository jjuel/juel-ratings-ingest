-- ============================================
-- Migration: Create all tables for juel_ratings
-- ============================================

-- 1. Teams
CREATE TABLE teams (
    id SERIAL PRIMARY KEY,              -- internal DB ID
    cfbd_id INT NOT NULL UNIQUE,        -- CFBD API ID
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
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    elevation TEXT,
    capacity BIGINT,
    construction_year INT,
    grass BOOLEAN,
    dome BOOLEAN,
    alternate_names TEXT[]
);

-- 2. Games
CREATE TABLE games (
    id SERIAL PRIMARY KEY,              -- internal DB ID
    cfbd_id BIGINT NOT NULL UNIQUE,     -- CFBD game ID
    season INT NOT NULL,
    week INT NOT NULL,
    season_type TEXT NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    home_team TEXT NOT NULL,
    home_points INT,
    away_team TEXT NOT NULL,
    away_points INT
);

-- Helpful indexes for common lookups
CREATE INDEX idx_games_season_week ON games (season, week);

-- 3. Drives
CREATE TABLE drives (
    id SERIAL PRIMARY KEY,
    cfbd_id TEXT NOT NULL UNIQUE,
    game_id INT NOT NULL REFERENCES games(id),
    offense_team_id INT NOT NULL REFERENCES teams(id),
    defense_team_id INT NOT NULL REFERENCES teams(id),
    offense_conference TEXT,
    defense_conference TEXT,
    drive_number INT NOT NULL,
    scoring BOOLEAN,
    start_period INT,
    start_yardline INT,
    start_yards_to_goal INT,
    start_time_minutes INT,
    start_time_seconds INT,
    end_period INT,
    end_yardline INT,
    end_yards_to_goal INT,
    end_time_minutes INT,
    end_time_seconds INT,
    elapsed_minutes INT,
    elapsed_seconds INT,
    plays INT,
    yards INT,
    drive_result TEXT,
    is_home_offense BOOLEAN,
    start_offense_score INT,
    start_defense_score INT,
    end_offense_score INT,
    end_defense_score INT
);

-- 4. Game Advanced Stats
CREATE TABLE game_advanced_stats (
    id SERIAL PRIMARY KEY,
    game_id INT NOT NULL REFERENCES games(id),
    team_id INT NOT NULL REFERENCES teams(id),
    opponent_id INT NOT NULL REFERENCES teams(id),
    offense_passing_plays_explosiveness DOUBLE PRECISION,
    offense_passing_plays_success_rate DOUBLE PRECISION,
    offense_passing_plays_total_ppa DOUBLE PRECISION,
    offense_passing_plays_ppa DOUBLE PRECISION,
    offense_rushing_plays_explosiveness DOUBLE PRECISION,
    offense_rushing_plays_success_rate DOUBLE PRECISION,
    offense_rushing_plays_total_ppa DOUBLE PRECISION,
    offense_rushing_plays_ppa DOUBLE PRECISION,
    offense_passing_downs_explosiveness DOUBLE PRECISION,
    offense_passing_downs_success_rate DOUBLE PRECISION,
    offense_passing_downs_ppa DOUBLE PRECISION,
    offense_standard_downs_explosiveness DOUBLE PRECISION,
    offense_standard_downs_success_rate DOUBLE PRECISION,
    offense_standard_downs_ppa DOUBLE PRECISION,
    offense_open_field_yards_total DOUBLE PRECISION,
    offense_open_field_yards DOUBLE PRECISION,
    offense_second_level_yards_total DOUBLE PRECISION,
    offense_second_level_yards DOUBLE PRECISION,
    offense_line_yards_total DOUBLE PRECISION,
    offense_line_yards DOUBLE PRECISION,
    offense_stuff_rate DOUBLE PRECISION,
    offense_power_success DOUBLE PRECISION,
    offense_explosiveness DOUBLE PRECISION,
    offense_success_rate DOUBLE PRECISION,
    offense_total_ppa DOUBLE PRECISION,
    offense_ppa DOUBLE PRECISION,
    offense_drives INT,
    offense_plays INT,
    defense_passing_plays_explosiveness DOUBLE PRECISION,
    defense_passing_plays_success_rate DOUBLE PRECISION,
    defense_passing_plays_total_ppa DOUBLE PRECISION,
    defense_passing_plays_ppa DOUBLE PRECISION,
    defense_rushing_plays_explosiveness DOUBLE PRECISION,
    defense_rushing_plays_success_rate DOUBLE PRECISION,
    defense_rushing_plays_total_ppa DOUBLE PRECISION,
    defense_rushing_plays_ppa DOUBLE PRECISION,
    defense_passing_downs_explosiveness DOUBLE PRECISION,
    defense_passing_downs_success_rate DOUBLE PRECISION,
    defense_passing_downs_ppa DOUBLE PRECISION,
    defense_standard_downs_explosiveness DOUBLE PRECISION,
    defense_standard_downs_success_rate DOUBLE PRECISION,
    defense_standard_downs_ppa DOUBLE PRECISION,
    defense_open_field_yards_total DOUBLE PRECISION,
    defense_open_field_yards DOUBLE PRECISION,
    defense_second_level_yards_total DOUBLE PRECISION,
    defense_second_level_yards DOUBLE PRECISION,
    defense_line_yards_total DOUBLE PRECISION,
    defense_line_yards DOUBLE PRECISION,
    defense_stuff_rate DOUBLE PRECISION,
    defense_power_success DOUBLE PRECISION,
    defense_explosiveness DOUBLE PRECISION,
    defense_success_rate DOUBLE PRECISION,
    defense_total_ppa DOUBLE PRECISION,
    defense_ppa DOUBLE PRECISION,
    defense_drives INT,
    defense_plays INT,
    raw JSONB,
    UNIQUE(game_id, team_id)
);
