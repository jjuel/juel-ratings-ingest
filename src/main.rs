use clap::{Parser, Subcommand, ValueEnum};
use dotenv::dotenv;
use sqlx::SqlitePool;

use anyhow::{Context, Ok, Result};

use crate::db::{drives::build_drive_map, games::build_game_map, pool, teams::build_team_name_map};

mod api;
mod db;

#[derive(Parser)]
#[command(version, about = "Ingesting CFDB data for Juel Ratings")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    All {
        #[arg(short, long, default_value = "2025")]
        year: u32,
        #[arg(short, long)]
        week: Option<u32>,
        #[arg(short, long)]
        season_type: Option<SeasonType>,
    },
    Teams {
        #[arg(short, long, default_value = "2025")]
        year: u32,
    },
    Games {
        #[arg(short, long, default_value = "2025")]
        year: u32,
        #[arg(short, long)]
        week: Option<u32>,
        #[arg(short, long)]
        season_type: Option<SeasonType>,
    },
    Drives {
        #[arg(short, long, default_value = "2025")]
        year: u32,
        #[arg(short, long)]
        week: Option<u32>,
        #[arg(short, long)]
        season_type: Option<SeasonType>,
    },
    Plays {
        #[arg(short, long, default_value = "2025")]
        year: u32,
        #[arg(short, long, default_value = "1")]
        week: u32,
        #[arg(short, long)]
        season_type: Option<SeasonType>,
    },
    AdvStats {
        #[arg(short, long, default_value = "2025")]
        year: u32,
        #[arg(short, long)]
        week: Option<u32>,
        #[arg(short, long)]
        season_type: Option<SeasonType>,
    },
    Havoc {
        #[arg(short, long, default_value = "2025")]
        year: u32,
        #[arg(short, long)]
        week: Option<u32>,
        #[arg(short, long)]
        season_type: Option<SeasonType>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SeasonType {
    Regular,
    Postseason,
    Both,
    AllStar,
    SpringRegular,
    SpringPostseason,
}

impl SeasonType {
    fn as_str(&self) -> &str {
        match self {
            Self::Regular => "regular",
            Self::Postseason => "postseason",
            Self::Both => "both",
            Self::AllStar => "allstar",
            Self::SpringRegular => "spring_regular",
            Self::SpringPostseason => "spring_postseason",
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    let pool = pool::create_pool().await?;

    match &cli.command {
        Commands::All {
            year,
            week,
            season_type,
        } => {
            let week = week.map(|w| w as i32);

            let scope = match week {
                Some(w) => format!("week {} of year {}", w, year),
                None => format!("year {}", year),
            };

            let season_type = season_type.map(|st| st.as_str().to_string());

            println!("\n🏈 Starting full data ingestion for {}\n", scope);
            let start = std::time::Instant::now();

            ingest_teams(&pool, *year as i32).await?;
            ingest_games(&pool, *year as i32, week, season_type.clone()).await?;
            ingest_drives(&pool, *year as i32, week, season_type.clone()).await?;

            // Plays require a week parameter
            if let Some(w) = week {
                ingest_plays(&pool, *year as i32, w, season_type.clone()).await?;
            }

            ingest_game_advanced_stats(&pool, *year as i32, week, season_type.clone()).await?;
            ingest_havoc(&pool, *year as i32, week, season_type.clone()).await?;

            let duration = start.elapsed();
            println!(
                "\n✨ All data ingested successfully in {:.2}s\n",
                duration.as_secs_f64()
            );
        }
        Commands::Teams { year } => {
            ingest_teams(&pool, *year as i32).await?;
        }
        Commands::Games {
            year,
            week,
            season_type,
        } => {
            let week = week.map(|w| w as i32);
            let season_type = season_type.map(|st| st.as_str().to_string());
            ingest_games(&pool, *year as i32, week, season_type).await?;
        }
        Commands::Drives {
            year,
            week,
            season_type,
        } => {
            let week = week.map(|w| w as i32);
            let season_type = season_type.map(|st| st.as_str().to_string());
            ingest_drives(&pool, *year as i32, week, season_type).await?;
        }
        Commands::Plays {
            year,
            week,
            season_type,
        } => {
            let season_type = season_type.map(|st| st.as_str().to_string());
            ingest_plays(&pool, *year as i32, *week as i32, season_type).await?;
        }
        Commands::AdvStats {
            year,
            week,
            season_type,
        } => {
            let week = week.map(|w| w as i32);
            let season_type = season_type.map(|st| st.as_str().to_string());
            ingest_game_advanced_stats(&pool, *year as i32, week, season_type).await?;
        }
        Commands::Havoc {
            year,
            week,
            season_type,
        } => {
            let week = week.map(|w| w as i32);
            let season_type = season_type.map(|st| st.as_str().to_string());
            ingest_havoc(&pool, *year as i32, week, season_type).await?;
        }
    }

    Ok(())
}

async fn ingest_teams(pool: &SqlitePool, year: i32) -> Result<usize> {
    let api_teams = api::teams::fetch(year).await.context(format!(
        "Failed to fetch teams from CFBD API for year {}",
        year
    ))?;

    let db_teams: Vec<db::Team> = api_teams.iter().map(db::mappings::map_team).collect();

    let stats = db::teams::upsert_batch(pool, &db_teams)
        .await
        .context(format!(
            "Failed to insert/update {} teams into database",
            db_teams.len()
        ))?;

    println!(
        "✓ Ingested {} teams ({} new, {} updated) for year {}",
        stats.ids.len(),
        stats.inserted,
        stats.updated,
        year
    );
    Ok(stats.ids.len())
}

async fn ingest_games(
    pool: &SqlitePool,
    year: i32,
    week: Option<i32>,
    season_type: Option<String>,
) -> Result<usize> {
    let scope = match week {
        Some(w) => format!("week {} of year {}", w, year),
        None => format!("year {}", year),
    };

    let api_games = api::games::fetch(year, week, season_type)
        .await
        .context(format!(
            "Failed to fetch games from CFBD API for {}\n",
            scope
        ))?;

    let db_games: Vec<db::Game> = api_games.iter().map(db::mappings::map_game).collect();

    let stats = db::games::upsert_batch(pool, &db_games)
        .await
        .context(format!(
            "Failed to insert/update {} games into database",
            db_games.len()
        ))?;

    println!(
        "✓ Ingested {} games ({} new, {} updated) for {}",
        stats.ids.len(),
        stats.inserted,
        stats.updated,
        scope
    );
    Ok(stats.ids.len())
}

async fn ingest_drives(
    pool: &SqlitePool,
    year: i32,
    week: Option<i32>,
    season_type: Option<String>,
) -> Result<usize> {
    let scope = match week {
        Some(w) => format!("week {} of year {}", w, year),
        None => format!("year {}", year),
    };

    let api_drives = api::drives::fetch(year, week, season_type)
        .await
        .context(format!(
            "Failed to fetch drives from CFBD API for {}",
            scope
        ))?;

    let teams_by_name = build_team_name_map(pool)
        .await
        .context("Failed to build team name lookup map")?;

    let games_by_cfbd_id = build_game_map(pool, year, week)
        .await
        .context(format!("Failed to build game ID lookup map for {}", scope))?;

    let fetched_count = api_drives.len();
    let db_drives: Vec<db::Drive> = api_drives
        .iter()
        .filter_map(|drive| {
            let game_id = games_by_cfbd_id.get(&drive.game_id)?;
            db::mappings::map_drive(drive, *game_id, &teams_by_name)
        })
        .collect();

    let skipped_count = fetched_count - db_drives.len();

    let stats = db::drives::upsert_batch(pool, &db_drives)
        .await
        .context(format!(
            "Failed to insert/update {} drives into database",
            db_drives.len()
        ))?;

    println!(
        "✓ Ingested {} drives ({} new, {} updated) for {} ({} skipped due to missing lookups)",
        stats.ids.len(),
        stats.inserted,
        stats.updated,
        scope,
        skipped_count
    );
    Ok(stats.ids.len())
}

async fn ingest_plays(
    pool: &SqlitePool,
    year: i32,
    week: i32,
    season_type: Option<String>,
) -> Result<usize> {
    let scope = format!("week {} of year {}", week, year);

    let api_plays = api::plays::fetch(year, week, season_type)
        .await
        .context(format!("Failed to fetch plays from CFBD API for {}", scope))?;

    let teams_by_name = build_team_name_map(pool)
        .await
        .context("Failed to build team name lookup map")?;

    let games_by_cfbd_id = build_game_map(pool, year, Some(week))
        .await
        .context(format!("Failed to build game ID lookup map for {}", scope))?;

    let drives_by_cfbd_id = build_drive_map(pool, year, Some(week))
        .await
        .context(format!("Failed to build drive ID lookup map for {}", scope))?;

    let fetched_count = api_plays.len();
    let db_plays: Vec<db::Play> = api_plays
        .iter()
        .filter_map(|play| {
            let game_id = play
                .game_id
                .and_then(|gid| games_by_cfbd_id.get(&(gid as i64)))?;
            db::mappings::map_play(play, *game_id, &teams_by_name, &drives_by_cfbd_id)
        })
        .collect();

    let skipped_count = fetched_count - db_plays.len();

    let stats = db::plays::upsert_batch(pool, &db_plays)
        .await
        .context(format!(
            "Failed to insert/update {} plays into database",
            db_plays.len()
        ))?;

    println!(
        "✓ Ingested {} plays ({} new, {} updated) for {} ({} skipped due to missing lookups)",
        stats.ids.len(),
        stats.inserted,
        stats.updated,
        scope,
        skipped_count
    );
    Ok(stats.ids.len())
}

async fn ingest_game_advanced_stats(
    pool: &SqlitePool,
    year: i32,
    week: Option<i32>,
    season_type: Option<String>,
) -> Result<usize> {
    let scope = match week {
        Some(w) => format!("week {} of year {}", w, year),
        None => format!("year {}", year),
    };

    let api_advanced_stats = api::game_advanced_stats::fetch(year, week, season_type)
        .await
        .context(format!(
            "Failed to fetch advanced stats from CFBD API for {}",
            scope
        ))?;

    let teams_by_name = build_team_name_map(pool)
        .await
        .context("Failed to build team name lookup map")?;

    let games_by_cfbd_id = build_game_map(pool, year, week)
        .await
        .context(format!("Failed to build game ID lookup map for {}", scope))?;

    let fetched_count = api_advanced_stats.len();
    let db_advanced_stats: Vec<db::GameAdvancedStats> = api_advanced_stats
        .iter()
        .filter_map(|advanced_stats| {
            let game_id = games_by_cfbd_id.get(&advanced_stats.game_id)?;
            db::mappings::map_game_advanced_stats(advanced_stats, *game_id, &teams_by_name)
        })
        .collect();

    let skipped_count = fetched_count - db_advanced_stats.len();

    let stats = db::game_advanced_stats::upsert_batch(pool, &db_advanced_stats)
        .await
        .context(format!(
            "Failed to insert/update {} game advanced stats into database",
            db_advanced_stats.len()
        ))?;

    println!(
        "✓ Ingested {} game advanced stats ({} new, {} updated) for {} ({} skipped due to missing lookups)",
        stats.ids.len(),
        stats.inserted,
        stats.updated,
        scope,
        skipped_count
    );
    Ok(stats.ids.len())
}

async fn ingest_havoc(
    pool: &SqlitePool,
    year: i32,
    week: Option<i32>,
    season_type: Option<String>,
) -> Result<usize> {
    let scope = match week {
        Some(w) => format!("week {} of year {}", w, year),
        None => format!("year {}", year),
    };

    let api_havoc = api::havoc::fetch(year, week, season_type)
        .await
        .context(format!(
            "Failed to fetch havoc stats from CFBD API for {}",
            scope
        ))?;

    let teams_by_name = build_team_name_map(pool)
        .await
        .context("Failed to build team name lookup map")?;

    let games_by_cfbd_id = build_game_map(pool, year, week)
        .await
        .context(format!("Failed to build game ID lookup map for {}", scope))?;

    let fetched_count = api_havoc.len();
    let db_havoc: Vec<db::Havoc> = api_havoc
        .iter()
        .filter_map(|havoc| {
            let game_id = games_by_cfbd_id.get(&havoc.game_id)?;
            db::mappings::map_havoc(havoc, *game_id, &teams_by_name)
        })
        .collect();

    let skipped_count = fetched_count - db_havoc.len();

    let stats = db::havoc::upsert_batch(pool, &db_havoc)
        .await
        .context(format!(
            "Failed to insert/update {} havoc stats into database",
            db_havoc.len()
        ))?;

    println!(
        "✓ Ingested {} havoc stats ({} new, {} updated) for {} ({} skipped due to missing lookups)",
        stats.ids.len(),
        stats.inserted,
        stats.updated,
        scope,
        skipped_count
    );
    Ok(stats.ids.len())
}
