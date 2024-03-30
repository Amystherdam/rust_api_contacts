use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};
use crate::database::contacts::Entity as Contacts;

pub async fn delete_contact(
  Path(id): Path<i32>,
  Extension(database): Extension<DatabaseConnection>
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
  let contact_name: String;

  match Contacts::find_by_id(id)
    .one(&database)
    .await
  {
    Ok(Some(contact)) => {
      contact_name = contact.name.clone();
      let contact = contact.into_active_model();

      match Contacts::delete(contact)
        .exec(&database)
        .await
      {
        Ok(_) => {
          Ok((StatusCode::OK,
            Json(serde_json::json!(
              { "message": { "id": format!("Contact {} with id {} deleted", contact_name, id)} }
            ))
          ))
        },
        Err(e) => {
          eprintln!("Failed to delete contact: {}", e);
          Err((StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to delete contact" }))
          ))
        }
      }
    },
    Ok(None) => {
      Err((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "error": format!("Contact with id {} not found", id) }))
      ))
    },
    Err(e) => {
      eprintln!("Failed to fetch contact: {}", e);
      Err((StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "Failed to fetch contact" }))
      ))
    }
  }
}