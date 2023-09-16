use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    Extension,
};
use common::{ErrorResponse, Todo, User};
use hyper::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::services::todo_service;

pub async fn get_todos(
    // Extension(user): Extension<User>,
    State(db): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let todo: Result<Todo, _> = Ok::<Todo, anyhow::Error>(Todo {
        id: 1,
        text: "asdf".into(),
        user_id: 1,
        is_completed: false,
    }); // todo_service::get_todo_by_id(&db, todo_id, user.id).await;

    match todo {
        Ok(todo) => Ok((StatusCode::OK, Json(json!({ "data": todo })))),
        Err(err) => {
            let err_response =
                ErrorResponse::new("error", &format!("Could not fetch todo: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}

pub async fn get_todo(
    //Extension(user): Extension<User>,
    State(db): State<PgPool>,
    Path(todo_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let todo: Result<Todo, _> = Ok::<Todo, anyhow::Error>(Todo {
        id: 1,
        text: "asdf".into(),
        user_id: 1,
        is_completed: false,
    }); // todo_service::get_todo_by_id(&db, todo_id, user.id).await;

    match todo {
        Ok(todo) => Ok((StatusCode::OK, Json(json!({ "data": todo })))),
        Err(err) => {
            let err_response =
                ErrorResponse::new("error", &format!("Could not fetch todo: {:?}", err), None);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)))
        }
    }
}
