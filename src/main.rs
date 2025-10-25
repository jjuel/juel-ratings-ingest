use clap::Parser;

use crate::api::game_advanced_stats::get_advanced_stats_by_year_and_week;

mod api;
mod cfbd;
mod db;

#[derive(Parser, Debug)]
#[command(version, about = "Ingesting CFDB data for Juel Ratings")]
struct Args {
    #[arg(short, long, default_value = "2025")]
    year: u32,
    #[arg(short, long, default_value = "1")]
    week: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let _ = get_advanced_stats_by_year_and_week(args.year, args.week).await;
}


