# Juel Ratings Ingest

A Rust-based CLI tool for ingesting college football data from the College Football Database (CFBD) API and storing it in PostgreSQL for Juel Ratings calculations.

## Features

- ✅ Fetch teams, games, drives, and advanced statistics from CFBD API
- ✅ Store data in PostgreSQL with referential integrity
- ✅ Idempotent upserts (safe to run multiple times)
- ✅ Smart upsert tracking (distinguishes inserts, updates, and unchanged data)
- ✅ Performance optimized (skips updating records when data is identical)
- ✅ Progress tracking and detailed error messages
- ✅ Individual or batch data ingestion modes

## Prerequisites

- **Rust** (1.70+) - [Install Rust](https://rustup.rs/)
- **PostgreSQL** (12+) - Running and accessible
- **CFBD API Key** - [Get your free API key](https://collegefootballdata.com/key)

## Setup

### 1. Clone and Build

```bash
git clone <repository-url>
cd juel-ratings-ingest
cargo build --release
```

### 2. Configure Environment

Create a `.env` file in the project root:

```bash
# CFBD API Key (required)
CFBD_API_KEY=your_api_key_here

# PostgreSQL connection string (required)
DATABASE_URL=postgresql://username:password@localhost/database_name
```

**Example:**
```bash
CFBD_API_KEY=abcd1234567890xyz
DATABASE_URL=postgresql://juel_ratings_app:password@localhost/juel_ratings
```

### 3. Run Database Migrations

Install sqlx-cli if you haven't already:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Run migrations to create tables:

```bash
sqlx migrate run
```

This creates four tables: `teams`, `games`, `drives`, and `game_advanced_stats`.

## Usage

The tool provides five CLI commands:

### Ingest All Data for a Week

```bash
cargo run --release -- all --year 2024 --week 10
```

**Example Output:**
```
🏈 Starting full data ingestion for week 10 of year 2024

✓ Ingested 133 teams (133 new, 0 updated) for year 2024
✓ Ingested 62 games (62 new, 0 updated) for week 10 of year 2024
✓ Ingested 1247 drives (1247 new, 0 updated) for week 10 of year 2024 (3 skipped due to missing lookups)
✓ Ingested 124 game advanced stats (124 new, 0 updated) for week 10 of year 2024 (0 skipped due to missing lookups)

✨ All data ingested successfully in 8.42s
```

The output shows a breakdown for each entity:
- **Total ingested**: Records that were either inserted or updated
- **X new**: Records that were newly inserted
- **Y updated**: Records that existed but had changed data
- **Skipped**: Records that couldn't be mapped due to missing foreign keys

When re-running the same ingestion with unchanged data, you'll see `0 new, 0 updated` since the smart upsert logic skips updating identical records.

### Ingest Individual Entities

**Teams (by year):**
```bash
cargo run --release -- teams --year 2024
```

**Games (by year and week):**
```bash
cargo run --release -- games --year 2024 --week 10
```

**Drives (by year and week):**
```bash
cargo run --release -- drives --year 2024 --week 10
```

**Advanced Stats (by year and week):**
```bash
cargo run --release -- adv-stats --year 2024 --week 10
```

### Using Defaults

All commands have default values (year=2025, week=1). You can run with defaults:

```bash
# Ingests all data for week 1 of 2025
cargo run --release -- all
```

### Short Flags

Use short flags for convenience:

```bash
cargo run --release -- all -y 2024 -w 10
```

## Data Dependencies

The tool handles foreign key relationships automatically:

- **Teams** → No dependencies (ingest first)
- **Games** → No dependencies (but best to ingest teams first)
- **Drives** → Requires teams and games (uses team names and game IDs)
- **Advanced Stats** → Requires teams and games (uses team names and game IDs)

**Recommended order**: Teams → Games → Drives → Advanced Stats

The `all` command handles this order automatically.

## Troubleshooting

### "relation does not exist" error

**Problem:** Database migrations haven't been run.

**Solution:**
```bash
sqlx migrate run
```

### "Failed to fetch from CFBD API"

**Problem:** Invalid or missing API key.

**Solution:**
- Check that `CFBD_API_KEY` is set in `.env`
- Verify your API key is valid at https://collegefootballdata.com/key
- Check your internet connection

### "Failed to connect to database"

**Problem:** PostgreSQL not running or wrong credentials.

**Solution:**
- Ensure PostgreSQL is running: `systemctl status postgresql`
- Verify `DATABASE_URL` in `.env` has correct username, password, and database name
- Test connection: `psql $DATABASE_URL`

### "X drives skipped due to missing lookups"

**Problem:** Drives reference teams or games not in the database.

**Solution:**
- Run `teams` command first to populate team data
- Run `games` command for the same week before running `drives`
- Or use the `all` command which handles the correct order

### Compilation errors about sqlx macros

**Problem:** sqlx needs database connection during compilation.

**Solution:**
- Ensure PostgreSQL is running
- Ensure migrations have been run
- Ensure `DATABASE_URL` is in `.env`
- Run `cargo clean && cargo build`

## Database Schema

The tool creates four main tables:

- **teams** - College football teams with location data (venues, coordinates)
- **games** - Game results by year, week, and season type
- **drives** - Individual drive data with scoring and field position
- **game_advanced_stats** - Advanced analytics (PPA, success rates, explosiveness, etc.)

All tables use:
- Auto-incrementing internal `id` as PRIMARY KEY
- CFBD's ID stored as `cfbd_id` with UNIQUE constraint
- Smart upserts with `ON CONFLICT DO UPDATE WHERE (data changed)`
  - Only updates records when data actually differs (performance optimization)
  - Uses PostgreSQL's `IS DISTINCT FROM` for NULL-safe comparisons
  - Tracks inserts vs updates using `xmax = 0` system column

## Performance Notes

- **API Rate Limits**: CFBD has rate limits. The tool makes sequential requests.
- **First Run**: Ingesting a full week takes 5-10 seconds depending on API response time.
- **Subsequent Runs**: Same duration (upserts update existing records).
- **Large Batches**: Consider adding delays between weeks if ingesting multiple weeks.

## Development

### Run in Debug Mode

```bash
cargo run -- all -y 2024 -w 10
```

### Run Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

## Project Structure

```
src/
  api/                  # CFBD API client and response structures
    client.rs          # HTTP client setup
    teams.rs           # Teams endpoint
    games.rs           # Games endpoint
    drives.rs          # Drives endpoint
    game_advanced_stats.rs  # Advanced stats endpoint
  db/                   # Database models and operations
    pool.rs            # Connection pool
    teams.rs           # Teams table with upsert
    games.rs           # Games table with upsert
    drives.rs          # Drives table with upsert
    game_advanced_stats.rs  # Advanced stats table with upsert
    mappings.rs        # API → DB struct conversions
  main.rs              # CLI entry point and orchestration
```

## Contributing

Issues and pull requests welcome!

## License

[Add your license here]

## Acknowledgments

Data provided by the [College Football Database](https://collegefootballdata.com/).
