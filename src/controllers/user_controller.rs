use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    Extension,
};
use hyper::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    models::{
        error::ErrorResponse,
        user::{CreateUserRequest, User},
    },
    services::user_service,
};

pub async fn get_users(
    Extension(user): Extension<User>,
    State(db): State<Arc<PgPool>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match user_service::get_all_users(&db).await {
        Ok(users) => Ok((
            StatusCode::OK,
            Json(json!({ "data": users, "totalCount": users.len() })),
        )),
        Err(err) => {
            let err_response =
                ErrorResponse::new("error", &format!("Could not fetch users: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}

pub async fn get_user(
    State(db): State<Arc<PgPool>>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match user_service::get_user_by_id(&db, user_id).await {
        Ok(user) => Ok((StatusCode::OK, Json(json!({ "data": user })))),
        Err(err) => {
            if err.to_string().contains("no rows returned") {
                let err_response = ErrorResponse::new("error", "user not found by id", None);
                return Err((StatusCode::NOT_FOUND, Json(err_response)));
            }

            let err_response =
                ErrorResponse::new("error", &format!("Could not fetch user: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}

pub async fn store_user(
    State(db): State<Arc<PgPool>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    if let Err(err) = req.validate() {
        let err_response = ErrorResponse::new("error", "validation of req failed", Some(&err));
        return Err((StatusCode::BAD_REQUEST, Json(err_response)));
    }

    let user_data = req;

    match user_service::register(&db, user_data).await {
        Ok(user_id) => Ok((
            StatusCode::CREATED,
            [("location", format!("/api/v1/users/{}", user_id))],
        )),
        Err(err) => {
            if err.to_string().contains("users_email_key") {
                let err_response = ErrorResponse::new("error", "email already in use", None);
                return Err((StatusCode::CONFLICT, Json(err_response)));
            }

            let err_response =
                ErrorResponse::new("error", &format!("Could not create user: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}
