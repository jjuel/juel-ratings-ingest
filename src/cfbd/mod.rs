pub mod game_advanced_stats;
pub mod games;
pub mod teams;
pub mod drives;

// Re-export commonly used types at the crate::cfbd::* level
pub use game_advanced_stats::GameAdvancedStats;
pub use drives::Drive;
pub use teams::Team;
