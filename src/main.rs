use db_seeder::{config::*, db::*, generators::*};

/// Main entry point of the application
/// Orchestrates the database seeding process
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup database connection
    let pool = setup_database(&get_database_url()).await?;

    // Generate and insert users
    let users = generate_users(NUM_USERS);
    let user_ids = insert_users(&pool, &users).await?;
    println!("Inserted {} users", user_ids.len());

    // Generate and insert posts
    let posts = generate_posts(NUM_POSTS, &users);
    let post_ids = insert_posts(&pool, &posts).await?;
    println!("Inserted {} posts", post_ids.len());

    // Generate and insert comments
    let comments = generate_comments(NUM_COMMENTS, &users, &posts);
    let comment_ids = insert_comments(&pool, &comments).await?;
    println!("Inserted {} comments", comment_ids.len());

    // Generate and insert interactions
    let interactions = generate_interactions(NUM_INTERACTIONS, &users, &posts);
    insert_interactions(&pool, &interactions).await?;
    println!("Inserted {} interactions", interactions.len());

    // Generate and insert recommendations
    let recommendations = generate_recommendations(NUM_RECOMMENDATIONS, &users, &posts);
    insert_recommendations(&pool, &recommendations).await?;
    println!("Inserted {} recommendations", recommendations.len());

    println!("Database populated successfully!");
    Ok(())
}
