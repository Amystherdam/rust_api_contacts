use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};
use crate::database::contacts::Entity as Contacts;

pub async fn delete_contact(
  Path(id): Path<i32>,
  Extension(database): Extension<DatabaseConnection>
) -> Result<(), StatusCode> {
  let contact = 
    if let Some(contact) = Contacts::find_by_id(id)
      .one(&database) 
      .await
      .map_err(|e| {
        eprintln!("Failed to fetch contact: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
      })? 
    {
        contact.into_active_model()
    } else {
      return Err(StatusCode::NOT_FOUND);
    };

    Contacts::delete(contact).exec(&database).await.map_err(|e| {
      eprintln!("Failed to delete contact: {}", e);
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
  

}