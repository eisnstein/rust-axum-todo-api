use anyhow::Result;
use common::{Todo, TodoId, UserId};
use sqlx::{PgPool, Row};

pub async fn get_all_todos(db: &PgPool, user_id: UserId) -> Result<Vec<Todo>> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(db)
        .await
        .map_err(|err| anyhow::anyhow!(err))
}

pub async fn get_todo_by_id(db: &PgPool, todo_id: TodoId, user_id: UserId) -> Result<Todo> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1 AND user_id = $2")
        .bind(todo_id)
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(|err| anyhow::anyhow!(err))
}
