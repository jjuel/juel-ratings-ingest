-- ============================================
-- Migration: Create havoc table
-- ============================================

CREATE TABLE havoc (
    id SERIAL PRIMARY KEY,
    game_id INT NOT NULL REFERENCES games(id),
    team_id INT NOT NULL REFERENCES teams(id),
    season INT NOT NULL,
    season_type TEXT,
    week INT,
    team TEXT NOT NULL,
    conference TEXT,
    opponent TEXT NOT NULL,
    opponent_conference TEXT,
    -- Offense stats
    offense_db_havoc_rate DOUBLE PRECISION,
    offense_front_seven_havoc_rate DOUBLE PRECISION,
    offense_havoc_rate DOUBLE PRECISION,
    offense_db_havoc_events DOUBLE PRECISION,
    offense_front_seven_havoc_events DOUBLE PRECISION,
    offense_total_havoc_events DOUBLE PRECISION,
    offense_total_plays DOUBLE PRECISION,
    -- Defense stats
    defense_db_havoc_rate DOUBLE PRECISION,
    defense_front_seven_havoc_rate DOUBLE PRECISION,
    defense_havoc_rate DOUBLE PRECISION,
    defense_db_havoc_events DOUBLE PRECISION,
    defense_front_seven_havoc_events DOUBLE PRECISION,
    defense_total_havoc_events DOUBLE PRECISION,
    defense_total_plays DOUBLE PRECISION,
    -- Constraints
    UNIQUE (game_id, team_id)
);

-- Helpful indexes for common lookups
CREATE INDEX idx_havoc_game_id ON havoc (game_id);
CREATE INDEX idx_havoc_team_id ON havoc (team_id);
CREATE INDEX idx_havoc_season_week ON havoc (season, week);
