use crate::db::helpers::{insert_items, insert_items_without_ids};
use crate::models::{Comment, Interaction, Post, Recommendation, User};
use crate::query::Query;
use sqlx::{Error as SqlxError, Pool, Postgres};
use uuid::Uuid;

/// Insert users into the database
pub async fn insert_users(pool: &Pool<Postgres>, users: &[User]) -> Result<Vec<Uuid>, SqlxError> {
    insert_items(pool, users, Query::InsertUser, |query_str, user| {
        sqlx::query(query_str)
            .bind(user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.role)
    })
    .await
}

/// Insert posts into the database
pub async fn insert_posts(pool: &Pool<Postgres>, posts: &[Post]) -> Result<Vec<Uuid>, SqlxError> {
    insert_items(pool, posts, Query::InsertPost, |query_str, post| {
        sqlx::query(query_str)
            .bind(post.id)
            .bind(post.author_id)
            .bind(&post.title)
            .bind(&post.content)
            .bind(&post.markdown_content)
    })
    .await
}

/// Insert comments into the database
pub async fn insert_comments(
    pool: &Pool<Postgres>,
    comments: &[Comment],
) -> Result<Vec<Uuid>, SqlxError> {
    insert_items(
        pool,
        comments,
        Query::InsertComment,
        |query_str, comment| {
            sqlx::query(query_str)
                .bind(comment.id)
                .bind(comment.post_id)
                .bind(comment.author_id)
                .bind(comment.parent_comment_id)
                .bind(&comment.content)
        },
    )
    .await
}

/// Insert interactions into the database
pub async fn insert_interactions(
    pool: &Pool<Postgres>,
    interactions: &[Interaction],
) -> Result<(), SqlxError> {
    insert_items_without_ids(
        pool,
        interactions,
        Query::InsertInteraction,
        |query_str, interaction| {
            sqlx::query(query_str)
                .bind(interaction.id)
                .bind(interaction.user_id)
                .bind(interaction.post_id)
                .bind(&interaction.interaction_type)
                .bind(interaction.timestamp)
        },
    )
    .await
}

/// Insert recommendations into the database
pub async fn insert_recommendations(
    pool: &Pool<Postgres>,
    recommendations: &[Recommendation],
) -> Result<(), SqlxError> {
    insert_items_without_ids(
        pool,
        recommendations,
        Query::InsertRecommendation,
        |query_str, recommendation| {
            sqlx::query(query_str)
                .bind(recommendation.id)
                .bind(recommendation.user_id)
                .bind(recommendation.post_id)
                .bind(recommendation.score)
        },
    )
    .await
}
