use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use common::{AuthenticateRequest, ErrorResponse};
use hyper::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::services::{jwt_service, password_service, user_service};

pub async fn authenticate(
    State(db): State<Arc<PgPool>>,
    Json(req): Json<AuthenticateRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    if let Err(err) = req.validate() {
        let err_response = ErrorResponse::new("error", "validation of req failed", Some(&err));
        return Err((StatusCode::BAD_REQUEST, Json(err_response)));
    }

    let maybe_user = user_service::get_user_by_email(&db, req.email.unwrap()).await;
    if maybe_user.is_err() {
        let err_response = ErrorResponse::new("error", "Eamil of password wrong", None);
        return Err((StatusCode::BAD_REQUEST, Json(err_response)));
    }

    let user = maybe_user.unwrap();

    match password_service::verify(req.password.unwrap(), user.password_hash.clone()).await {
        Err(_e) => {
            let err_response = ErrorResponse::new("error", "Email or password wrong", None);
            return Err((StatusCode::BAD_REQUEST, Json(err_response)));
        }
        Ok(false) => {
            let err_response = ErrorResponse::new("error", "Password wrong", None);
            return Err((StatusCode::BAD_REQUEST, Json(err_response)));
        }
        Ok(_) => {}
    }

    match jwt_service::create(&user).await {
        Err(err) => {
            let err_response = ErrorResponse::new("error", &err.to_string(), None);
            return Err((StatusCode::BAD_REQUEST, Json(err_response)));
        }
        Ok(token) => Ok((StatusCode::OK, Json(json!({ "token": token })))),
    }
}
