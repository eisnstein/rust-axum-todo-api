use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};
use validator::Validate;

pub type UserId = i32;

#[derive(Deserialize, Serialize, Debug, FromRow, Clone)]
#[allow(non_snake_case)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: UserId,
    pub exp: usize,
}
