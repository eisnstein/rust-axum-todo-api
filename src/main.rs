use std::error::Error;
use std::{env, sync::Arc};

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use hyper::Server;
use sqlx::postgres::PgPoolOptions;
use tracing::Level;

pub mod controllers;
pub mod middlewares;
pub mod models;
pub mod services;

use controllers::{auth_controller, todo_controller, user_controller};
use middlewares::admin_middlware::admin;
use middlewares::auth_middlware::auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    dotenvy::dotenv()?;

    let connection_string = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await;

    let pool = match pool {
        Ok(pool) => {
            tracing::info!("Connected to db...");
            Arc::new(pool)
        }
        Err(err) => {
            tracing::error!("Could not connect to db: {:?}", err);
            std::process::exit(1);
        }
    };

    // tracing::info!("Running migrations...");
    // sqlx::migrate!().run(&pool).await?;

    let user_routes = Router::new()
        .route("/", get(user_controller::get_users))
        .route("/:user_id", get(user_controller::get_user))
        .route_layer(middleware::from_fn(admin))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth))
        .route("/", post(user_controller::store_user));

    let todo_routes = Router::new()
        .route("/", get(todo_controller::get_todos))
        .route("/:todo_id", get(todo_controller::get_todo))
        .route("/", post(todo_controller::store_todo))
        .route("/:todo_id", post(todo_controller::update_todo))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth));

    let auth_routes = Router::new().route("/token", post(auth_controller::authenticate));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/todos", todo_routes)
        .nest("/auth", auth_routes);

    let router = Router::new().nest("/api/v1", api_routes).with_state(pool);

    let addr = "127.0.0.1:3000".parse()?;
    let server = Server::bind(&addr).serve(router.into_make_service());

    tracing::info!("Listening on {addr}...");

    server.await?;

    Ok(())
}
