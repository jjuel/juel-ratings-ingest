-- ============================================
-- Migration: Create plays table
-- ============================================

CREATE TABLE plays (
    id SERIAL PRIMARY KEY,
    cfbd_id TEXT NOT NULL UNIQUE,
    cfbd_drive_id TEXT,
    drive_id INT REFERENCES drives(id),
    game_id INT NOT NULL REFERENCES games(id),
    drive_number INT,
    play_number INT,
    offense TEXT,
    offense_team_id INT NOT NULL REFERENCES teams(id),
    offense_conference TEXT,
    offense_score INT,
    defense TEXT,
    defense_team_id INT NOT NULL REFERENCES teams(id),
    defense_conference TEXT,
    defense_score INT,
    home TEXT,
    away TEXT,
    period INT,
    clock_minutes INT,
    clock_seconds INT,
    offense_timeouts INT,
    defense_timeouts INT,
    yardline INT,
    yards_to_goal INT,
    down INT,
    distance INT,
    yards_gained INT,
    scoring BOOLEAN,
    play_type TEXT,
    play_text TEXT,
    ppa DOUBLE PRECISION,
    wallclock TEXT
);

-- Helpful indexes for common lookups
CREATE INDEX idx_plays_game_id ON plays (game_id);
CREATE INDEX idx_plays_drive_id ON plays (drive_id);
CREATE INDEX idx_plays_offense_team_id ON plays (offense_team_id);
CREATE INDEX idx_plays_defense_team_id ON plays (defense_team_id);
