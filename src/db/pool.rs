use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;
use std::env;
use std::str::FromStr;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .foreign_keys(true)
        .busy_timeout(std::time::Duration::from_secs(5))
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;

    bootstrap_legacy_schema(&pool).await?;
    MIGRATOR.run(&pool).await?;

    Ok(pool)
}

async fn bootstrap_legacy_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let applied_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
        .fetch_one(pool)
        .await?;

    if applied_count > 0 {
        return Ok(());
    }

    let legacy_object_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM sqlite_master
         WHERE name IN ('teams', 'games', 'drives', 'game_advanced_stats', 'plays', 'havoc')",
    )
    .fetch_one(pool)
    .await?;

    if legacy_object_count == 0 {
        return Ok(());
    }

    if let Some(migration) = MIGRATOR.iter().next() {
        sqlx::query(
            "INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time)
             VALUES (?, ?, 1, ?, 0)",
        )
        .bind(migration.version)
        .bind(migration.description.as_ref())
        .bind(migration.checksum.as_ref())
        .execute(pool)
        .await?;
    }

    Ok(())
}
