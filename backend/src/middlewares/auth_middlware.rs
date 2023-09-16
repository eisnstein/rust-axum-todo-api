use std::sync::Arc;

use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use sqlx::PgPool;

use crate::services::user_service::authorize_current_user;

pub async fn auth<B>(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(db): State<Arc<PgPool>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    if let Some(current_user) = authorize_current_user(&db, bearer.token()).await {
        req.extensions_mut().insert(current_user);
        let res = next.run(req).await;
        Ok(res)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
