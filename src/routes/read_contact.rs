use axum::{
  extract::Path, 
  Extension,
  Json, 
  http::StatusCode,
  response::IntoResponse
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use crate::database::contacts::Entity as Contacts;

#[derive(Serialize)]
pub struct ResponseContact {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub phone: String,
}

pub async fn read_contact(
  Path(id): Path<i32>, 
  Extension(database): Extension<DatabaseConnection>
) -> impl IntoResponse {
  let contact = Contacts::find_by_id(id).one(&database).await.unwrap();

  if let Some(contact) = contact {
    Ok(Json(ResponseContact {
      id: contact.id,
      name: contact.name,
      email: contact.email,
      phone: contact.phone,
    }))
  } else {
    Err((StatusCode::NOT_FOUND, Json(serde_json::json!({
      "errors": {
        "id": format!("Contact with id {} not found", id)
      }
    }))))
  }
}