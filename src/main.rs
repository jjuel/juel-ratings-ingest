use crate::api::game_advanced_stats::get_advanced_stats_by_year_and_week;

mod api;
mod cfbd;
mod db;

#[tokio::main]
async fn main() {
    let _ = get_advanced_stats_by_year_and_week(2025, 1).await;
}
