/// Enum representing SQL queries for each database operation
pub enum Query {
    /// Insert a user record
    InsertUser,
    /// Insert a post record
    InsertPost,
    /// Insert a comment record
    InsertComment,
    /// Insert an interaction record
    InsertInteraction,
    /// Insert a recommendation record
    InsertRecommendation,
}

impl Query {
    /// Returns the SQL query string for the given query type
    pub fn as_str(&self) -> &str {
        match self {
            Query::InsertUser => {
                r#"
                INSERT INTO global.users (id, username, email, password_hash, role)
                VALUES ($1, $2, $3, $4, $5)
            "#
            }
            Query::InsertPost => {
                r#"
                INSERT INTO global.posts (id, author_id, title, content, markdown_content)
                VALUES ($1, $2, $3, $4, $5)
            "#
            }
            Query::InsertComment => {
                r#"
                INSERT INTO global.comments (id, post_id, author_id, parent_comment_id, content)
                VALUES ($1, $2, $3, $4, $5)
            "#
            }
            Query::InsertInteraction => {
                r#"
                INSERT INTO global.interactions (id, user_id, post_id, interaction_type, timestamp)
                VALUES ($1, $2, $3, $4, $5)
            "#
            }
            Query::InsertRecommendation => {
                r#"
                INSERT INTO global.recommendations (id, user_id, post_id, score)
                VALUES ($1, $2, $3, $4)
            "#
            }
        }
    }
}
