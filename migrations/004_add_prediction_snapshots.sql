CREATE TABLE IF NOT EXISTS prediction_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    season INTEGER NOT NULL,
    week INTEGER NOT NULL,
    season_type TEXT NOT NULL,
    model_name TEXT NOT NULL,
    model_version TEXT NOT NULL,
    ratings_snapshot_id INTEGER NOT NULL REFERENCES rating_snapshots(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (season, week, season_type, model_name, model_version, ratings_snapshot_id)
);

CREATE INDEX IF NOT EXISTS idx_prediction_snapshots_lookup
    ON prediction_snapshots (season, week, season_type, model_name, model_version, ratings_snapshot_id);

CREATE TABLE IF NOT EXISTS game_predictions (
    snapshot_id INTEGER NOT NULL REFERENCES prediction_snapshots(id) ON DELETE CASCADE,
    game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    away_team TEXT NOT NULL,
    home_team TEXT NOT NULL,
    predicted_away_score REAL NOT NULL,
    predicted_home_score REAL NOT NULL,
    predicted_total REAL NOT NULL,
    favorite TEXT NOT NULL,
    spread REAL NOT NULL,
    away_win_prob REAL NOT NULL,
    home_win_prob REAL NOT NULL,
    away_rating REAL NOT NULL,
    home_rating REAL NOT NULL,
    neutral_site INTEGER NOT NULL,
    PRIMARY KEY (snapshot_id, game_id)
);

CREATE INDEX IF NOT EXISTS idx_game_predictions_game_id ON game_predictions (game_id);
CREATE INDEX IF NOT EXISTS idx_game_predictions_away_team ON game_predictions (away_team);
CREATE INDEX IF NOT EXISTS idx_game_predictions_home_team ON game_predictions (home_team);
