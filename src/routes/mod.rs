use axum::{
  body::Body, 
  routing::{get, post, patch, delete}, 
  Extension, Router
};
use sea_orm::DatabaseConnection;

pub mod read_contact;
pub mod read_contacts;
pub mod create_contact;
pub mod update_contact;
pub mod delete_contact;

use read_contact::read_contact;
use read_contacts::read_contacts;
use create_contact::create_contact;
use update_contact::update_contact;
use delete_contact::delete_contact;



pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
  Router::new()
    .route("/", get(read_contacts))
    .route("/contacts", get(read_contacts))
    .route("/contacts", post(create_contact))
    .route("/contacts/:id", get(read_contact))
    .route("/contacts/:id", patch(update_contact))
    .route("/contacts/:id", delete(delete_contact))
    .layer(Extension(database))
}