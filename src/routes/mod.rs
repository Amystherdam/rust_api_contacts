
use axum::{body::Body, Router};
use sea_orm::DatabaseConnection;

pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
  Router::new()
}