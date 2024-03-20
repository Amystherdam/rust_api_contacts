use axum::{
  body::Body, 
  routing::{get, post, patch, delete}, 
  Extension, Router
};
use sea_orm::DatabaseConnection;

mod read_contacts;
mod create_contact;

use read_contacts::read_contacts;
use create_contact::create_contact;



pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
  Router::new()
    .route("/", get(read_contacts))
    .route("/contacts", get(read_contacts))
    .route("/contacts", post(create_contact))
    // .route("/contacts/:id", get(todo!()))
    // .route("/contacts/:id", patch(todo!()))
    // .route("/contacts/:id", delete(todo!()))
    .layer(Extension(database))
}