use crate::generators::helpers::{
    choose_random_id, generate_items, generate_password, hash_password,
};
use crate::models::{Comment, Interaction, Post, Recommendation, User};
use chrono::{Duration, Utc};
use rand::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

/// Generates `count` random users
pub fn generate_users(count: usize) -> Vec<User> {
    let mut rng = thread_rng();
    generate_items(count, |i| {
        // Generate a UUID and use part of it to create a unique username
        let id = Uuid::new_v4();
        let username = format!(
            "user_{}",
            id.as_simple()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>()
        );
        let email = format!("{}@example.com", username);
        let password = generate_password(&mut rng, 12);
        let hashed_password = hash_password(&password);

        User {
            id,
            username,
            email,
            password_hash: hashed_password,
            role: if i == 0 {
                "admin".to_string()
            } else {
                "user".to_string()
            },
        }
    })
}

/// Generates `count` random posts
pub fn generate_posts(count: usize, users: &[User]) -> Vec<Post> {
    let mut rng = thread_rng();
    generate_items(count, |i| {
        let title = format!("Post title {}", i);
        let content = format!("This is the content of post {}. It's very interesting!", i);
        let author_id = choose_random_id(&mut rng, &users.iter().map(|u| u.id).collect::<Vec<_>>());

        Post {
            id: Uuid::new_v4(),
            title,
            content,
            author_id,
            markdown_content: format!(
                "# Post title {}\n\nThis is the content of post {}. It's very interesting!",
                i, i
            ),
        }
    })
}

/// Generates `count` random comments
pub fn generate_comments(count: usize, users: &[User], posts: &[Post]) -> Vec<Comment> {
    let mut rng = thread_rng();

    // Step 1: Generate all comments without parent references
    let mut comments = generate_items(count, |i| {
        let content = format!("This is comment {}!", i);
        let author_id = choose_random_id(&mut rng, &users.iter().map(|u| u.id).collect::<Vec<_>>());
        let post_id = choose_random_id(&mut rng, &posts.iter().map(|p| p.id).collect::<Vec<_>>());

        Comment {
            id: Uuid::new_v4(),
            content,
            author_id,
            post_id,
            parent_comment_id: None,
        }
    });

    // Step 2: Update some comments to have parent references
    for i in 1..comments.len() {
        if rng.gen_bool(0.3) {
            // Choose a random comment from those already generated as parent
            let parent_idx = rng.gen_range(0..i); // Only use comments with index less than current
            comments[i].parent_comment_id = Some(comments[parent_idx].id);
        }
    }

    comments
}

/// Generates `count` random interactions
pub fn generate_interactions(count: usize, users: &[User], posts: &[Post]) -> Vec<Interaction> {
    let mut rng = thread_rng();
    generate_items(count, |_| {
        let user_id = choose_random_id(&mut rng, &users.iter().map(|u| u.id).collect::<Vec<_>>());
        let post_id = choose_random_id(&mut rng, &posts.iter().map(|p| p.id).collect::<Vec<_>>());
        let interaction_type = if rng.gen_bool(0.7) { "like" } else { "share" };
        let timestamp = Utc::now() - Duration::hours(rng.gen_range(0..48));

        Interaction {
            id: Uuid::new_v4(),
            user_id,
            post_id,
            interaction_type: interaction_type.to_string(),
            timestamp,
        }
    })
}

/// Generates `count` random recommendations
pub fn generate_recommendations(
    count: usize,
    users: &[User],
    posts: &[Post],
) -> Vec<Recommendation> {
    let mut rng = thread_rng();
    generate_items(count, |_| {
        let user_id = choose_random_id(&mut rng, &users.iter().map(|u| u.id).collect::<Vec<_>>());
        let post_id = choose_random_id(&mut rng, &posts.iter().map(|p| p.id).collect::<Vec<_>>());

        // Generate a random score between 0.0 and 5.0 with one decimal place
        let score_float = (rng.gen_range(0..50) as f32) / 10.0;
        let score = Decimal::from_str(&format!("{:.1}", score_float)).unwrap();

        Recommendation {
            id: Uuid::new_v4(),
            user_id,
            post_id,
            score,
        }
    })
}
