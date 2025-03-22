use crate::models::Identifiable;
use crate::query::Query;
use sqlx::{Error as SqlxError, Pool, Postgres};
use uuid::Uuid;

/// Generic function to insert items and return their IDs
pub(crate) async fn insert_items<'a, T, F>(
    pool: &'a Pool<Postgres>,
    items: &'a [T],
    query: Query,
    bind_fn: F,
) -> Result<Vec<Uuid>, SqlxError>
where
    T: Identifiable,
    F: for<'b> Fn(&'b str, &'b T) -> sqlx::query::Query<'b, Postgres, sqlx::postgres::PgArguments>,
{
    let mut tx = pool.begin().await?;
    let mut item_ids = Vec::with_capacity(items.len());
    let query_str = query.as_str();

    for item in items {
        bind_fn(query_str, item).execute(&mut *tx).await?;
        item_ids.push(item.id());
    }

    tx.commit().await?;
    Ok(item_ids)
}

/// Generic function to insert items without returning IDs
pub(crate) async fn insert_items_without_ids<T, F>(
    pool: &Pool<Postgres>,
    items: &[T],
    query: Query,
    bind_fn: F,
) -> Result<(), SqlxError>
where
    F: for<'a> Fn(&'a str, &'a T) -> sqlx::query::Query<'a, Postgres, sqlx::postgres::PgArguments>,
{
    let mut tx = pool.begin().await?;
    let query_str = query.as_str();

    for item in items {
        bind_fn(query_str, item).execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(())
}
