use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use super::user::UserId;

pub type TodoId = i32;

#[derive(Deserialize, Serialize, Clone, Debug, FromRow)]
pub struct Todo {
    pub id: TodoId,
    pub todo: String,
    pub is_completed: bool,
    pub user_id: UserId,
}

#[derive(Deserialize, Validate, Debug)]
pub struct CreateTodoRequest {
    #[validate(required)]
    pub todo: Option<String>,
}

#[derive(Deserialize, Validate, Debug)]
pub struct UpdateTodoRequest {
    pub todo: Option<String>,
    pub is_completed: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TodoResponse {
    pub id: i32,
    pub todo: String,
    pub is_completed: bool,
}

impl TodoResponse {
    pub fn of(todo: Todo) -> TodoResponse {
        TodoResponse {
            id: todo.id,
            todo: todo.todo,
            is_completed: todo.is_completed,
        }
    }
}
