use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Trait for types that have a UUID identifier
pub trait Identifiable {
    fn id(&self) -> Uuid;
}

/// Represents a user in the blog system
#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

impl Identifiable for User {
    fn id(&self) -> Uuid {
        self.id
    }
}

/// Represents a blog post authored by a user
#[derive(Debug)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub markdown_content: String,
}

impl Identifiable for Post {
    fn id(&self) -> Uuid {
        self.id
    }
}

/// Represents a comment on a blog post
#[derive(Debug)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub content: String,
}

impl Identifiable for Comment {
    fn id(&self) -> Uuid {
        self.id
    }
}

/// Represents a user interaction with a post (view, like, share, etc.)
#[derive(Debug)]
pub struct Interaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub interaction_type: String,
    pub timestamp: DateTime<Utc>,
}

impl Identifiable for Interaction {
    fn id(&self) -> Uuid {
        self.id
    }
}

/// Represents a post recommendation for a user
#[derive(Debug)]
pub struct Recommendation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub score: Decimal,
}

impl Identifiable for Recommendation {
    fn id(&self) -> Uuid {
        self.id
    }
}
