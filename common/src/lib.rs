use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};
use validator::{Validate, ValidationErrors};

pub type UserId = i32;
pub type TodoId = i32;

#[derive(Deserialize, Serialize, Debug, FromRow)]
#[allow(non_snake_case)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub password_hash: String,
    #[serde(rename = "updateAt")]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Validate, Debug)]
pub struct CreateUserRequest {
    #[validate(email(message = "invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "password needs to be at least 8 chars long"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct AuthenticateRequest {
    #[validate(required)]
    pub email: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserResponse {
    pub id: UserId,
    pub email: String,
}

impl UserResponse {
    pub fn of(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            email: user.email,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub errors: Option<HashMap<String, String>>,
}

impl ErrorResponse {
    pub fn new(status: &str, message: &str, errors: Option<&ValidationErrors>) -> Self {
        let errors_map = match errors {
            Some(validation_errors) => {
                let mut map: HashMap<String, String> = HashMap::new();

                for (field, errors) in validation_errors.field_errors() {
                    let err_str = errors
                        .iter()
                        .map(|ve| match &ve.message {
                            Some(msg) => msg.clone().into_owned(),
                            None => ve.code.clone().into_owned(),
                        })
                        .collect::<Vec<String>>()
                        .join(",");

                    map.insert(field.into(), err_str);
                }

                Some(map)
            }
            None => None,
        };

        ErrorResponse {
            status: status.into(),
            message: message.into(),
            errors: errors_map,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: UserId,
    pub exp: usize,
}

#[derive(Deserialize, Serialize, Clone, Debug, FromRow)]
pub struct Todo {
    pub id: TodoId,
    pub text: String,
    pub is_completed: bool,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CreateTodoRequest {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TodoResponse {
    pub id: i32,
    pub text: String,
    pub is_completed: bool,
}

impl TodoResponse {
    pub fn of(todo: Todo) -> TodoResponse {
        TodoResponse {
            id: todo.id,
            text: todo.text,
            is_completed: todo.is_completed,
        }
    }
}
