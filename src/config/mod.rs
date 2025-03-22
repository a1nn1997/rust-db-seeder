use dotenvy::dotenv;
use std::env;

/// Get database connection URL from environment or use default
pub fn get_database_url() -> String {
    dotenv().ok(); // Load .env file if it exists
    env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL environment variable not set")
    })
}

/// Number of users to generate
pub const NUM_USERS: usize = 20;

/// Number of posts to generate
pub const NUM_POSTS: usize = 50;

/// Number of comments to generate
pub const NUM_COMMENTS: usize = 100;

/// Number of user interactions to generate
pub const NUM_INTERACTIONS: usize = 200;

/// Number of recommendations to generate
pub const NUM_RECOMMENDATIONS: usize = 100;
