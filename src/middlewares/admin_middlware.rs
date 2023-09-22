use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};

use crate::models::user::User;

pub async fn admin<B>(
    Extension(user): Extension<User>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    if user.is_admin {
        let res = next.run(req).await;
        Ok(res)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
