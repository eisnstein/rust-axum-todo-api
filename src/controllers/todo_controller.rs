use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    Extension,
};
use hyper::StatusCode;
use serde_json::{json, Value};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    models::{error::ErrorResponse, todo::CreateTodoRequest, user::User},
    services::todo_service,
};

pub async fn get_todos(
    State(db): State<Arc<PgPool>>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let todos = todo_service::get_all_todos(&db, user.id).await;

    match todos {
        Ok(todos) => Ok((StatusCode::OK, Json(json!({ "data": todos })))),
        Err(err) => {
            let err_response =
                ErrorResponse::new("error", &format!("Could not fetch todo: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}

pub async fn get_todo(
    Extension(user): Extension<User>,
    State(db): State<Arc<PgPool>>,
    Path(todo_id): Path<i32>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let todo = todo_service::get_todo_by_id(&db, todo_id, user.id).await;

    match todo {
        Ok(todo) => Ok((StatusCode::OK, Json(json!({ "data": todo })))),
        Err(err) => {
            let err_response =
                ErrorResponse::new("error", &format!("Could not fetch todo: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}

pub async fn store_todo(
    Extension(user): Extension<User>,
    State(db): State<Arc<PgPool>>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    if let Err(err) = req.validate() {
        let err_response = ErrorResponse::new("error", "validation of req failed", Some(&err));
        return Err((StatusCode::BAD_REQUEST, Json(err_response)));
    }

    let todo_data = req;

    match todo_service::create_todo(&db, todo_data, user.id).await {
        Ok(todo_id) => Ok((
            StatusCode::CREATED,
            [("location", format!("/api/v1/todos/{todo_id}",))],
        )),
        Err(err) => {
            let err_response =
                ErrorResponse::new("error", &format!("Could not create todo: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}
