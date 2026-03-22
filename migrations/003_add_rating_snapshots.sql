CREATE TABLE IF NOT EXISTS rating_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    season INTEGER NOT NULL,
    through_week INTEGER NOT NULL,
    ratings_scope TEXT NOT NULL CHECK (ratings_scope IN ('regular_only', 'all_games', 'postseason_only')),
    model_name TEXT NOT NULL,
    model_version TEXT NOT NULL,
    blend_alpha REAL NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (season, through_week, ratings_scope, model_name, model_version)
);

CREATE INDEX idx_rating_snapshots_lookup
    ON rating_snapshots (season, through_week, ratings_scope, model_name, model_version);

CREATE TABLE IF NOT EXISTS team_ratings (
    snapshot_id INTEGER NOT NULL REFERENCES rating_snapshots(id) ON DELETE CASCADE,
    team TEXT NOT NULL,
    classification TEXT,
    conference TEXT,
    games INTEGER,
    drives_per_game REAL,
    off_rating REAL,
    def_rating REAL,
    sp_efficiency REAL,
    massey_results REAL,
    blended_rating REAL,
    rank INTEGER,
    success_rate REAL,
    explosiveness REAL,
    sp_component REAL,
    massey_component REAL,
    PRIMARY KEY (snapshot_id, team)
);

CREATE INDEX idx_team_ratings_team ON team_ratings (team);
CREATE INDEX idx_team_ratings_snapshot_rank ON team_ratings (snapshot_id, rank);
