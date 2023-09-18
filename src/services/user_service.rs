use anyhow::Result;
use sqlx::{PgPool, Row};

use crate::{
    models::user::{CreateUserRequest, User, UserId},
    services::password_service,
};

use super::jwt_service;

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db)
        .await
        .map_err(|err| anyhow::anyhow!(err))
}

pub async fn get_user_by_id(db: &PgPool, user_id: i32) -> Result<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(|err| anyhow::anyhow!(err))
}

pub async fn get_user_by_email(db: &PgPool, email: String) -> Result<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&email)
        .fetch_one(db)
        .await
        .map_err(|err| anyhow::anyhow!(err))
}

pub async fn register(db: &PgPool, user_data: CreateUserRequest) -> Result<UserId> {
    let password_hash = password_service::generate_password_hash(user_data.password).await?;

    let result =
        sqlx::query("INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id")
            .bind(user_data.email)
            .bind(password_hash)
            .fetch_one(db)
            .await
            .map_err(|err| anyhow::anyhow!(err));

    match result {
        Ok(row) => Ok(row.get::<UserId, _>(0)),
        Err(e) => Err(e),
    }
}

pub async fn authorize_current_user(db: &PgPool, jwt_token: &str) -> Option<User> {
    match jwt_service::verify(jwt_token).await {
        Ok(claims) => {
            let maybe_user = get_user_by_id(&db, claims.sub).await;
            maybe_user.ok()
        }
        Err(e) => None,
    }
}
