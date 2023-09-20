use anyhow::Result;
use sqlx::{PgPool, Row};

use crate::models::{
    todo::{CreateTodoRequest, Todo, TodoId, UpdateTodoRequest},
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

pub async fn update_todo(
    db: &PgPool,
    todo_data: UpdateTodoRequest,
    todo_id: TodoId,
    user_id: UserId,
) -> Result<()> {
    let existing_todo = get_todo_by_id(&db, todo_id, user_id).await;

    if existing_todo.is_err() {
        return Err(existing_todo.unwrap_err());
    }

    let et = existing_todo.unwrap();

    let todo = todo_data.todo.unwrap_or(et.todo);
    let is_completed = todo_data.is_completed.unwrap_or(et.is_completed);

    let result =
        sqlx::query("UPDATE todos SET todo = $1, is_completed = $2 WHERE id = $3 RETURNING id")
            .bind(todo)
            .bind(is_completed)
            .bind(todo_id)
            .fetch_one(db)
            .await
            .map_err(|err| anyhow::anyhow!(err));

    match result {
        Ok(_row) => Ok(()),
        Err(e) => Err(e),
    }
}
