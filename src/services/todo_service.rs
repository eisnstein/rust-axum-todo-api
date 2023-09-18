use anyhow::Result;
use sqlx::{PgPool, Row};

use crate::models::{
    todo::{CreateTodoRequest, Todo, TodoId},
    user::UserId,
};

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

pub async fn create_todo(
    db: &PgPool,
    todo_data: CreateTodoRequest,
    user_id: UserId,
) -> Result<TodoId> {
    let result = sqlx::query(
        "INSERT INTO todos (todo, is_completed, user_id) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(todo_data.todo)
    .bind(false)
    .bind(user_id)
    .fetch_one(db)
    .await
    .map_err(|err| anyhow::anyhow!(err));

    match result {
        Ok(row) => Ok(row.get::<TodoId, _>(0)),
        Err(e) => Err(e),
    }
}
