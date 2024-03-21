use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde::Deserialize;
use validator::Validate;
use crate::database::contacts::{self, Entity as Contacts};
use sea_orm::{
  entity::prelude::DateTime, 
  ColumnTrait, 
  DatabaseConnection, 
  EntityTrait, 
  IntoActiveModel, 
  QueryFilter, 
  Set
};
use chrono::{NaiveDateTime, Utc};
use regex::Regex;



#[derive(Deserialize, Validate)]
pub struct RequestContact {
  #[validate(
    length(min = 3, max = 100, message = "Name must be between 3 and 100 characters"), 
    regex(path = Regex::new(r"^[a-zA-Z\s.']+$").unwrap(), message = "Name must only contain letters and spaces")
  )]
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
  )]
  pub name: Option<String>,

  #[validate(email(message = "Invalid email"))]
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
  )]
  pub email: Option<String>,

  #[validate(
    length(equal = 11, message = "Phone must be 11 characters"),
    regex(path = Regex::new(r"^\d{11}$").unwrap(), message = "Phone must only contain numbers")
  )]
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
  )]
  pub phone: Option<String>,
}

pub async fn update_contact(
  Path(id): Path<i32>,
  Extension(database): Extension<DatabaseConnection>,
  Json(request_contact): Json<RequestContact>
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
  if let Err(errors) = request_contact.validate() {
    return Err((
      StatusCode::BAD_REQUEST,
      Json(serde_json::json!({ "errors": errors }))
    ));
    
  }

  let mut contact = 
    if let Some(contact) = Contacts::find_by_id(id)
      .one(&database)
      .await
      .map_err(|e| {
        eprintln!("Failed to fetch contact: {}", e);
        (
          StatusCode::INTERNAL_SERVER_ERROR, 
          Json(serde_json::json!({ "error": "Failed to fetch contact" }))
        )})?
    {
      contact.into_active_model()
    } else {
      return Err((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "error": format!("Contact with id {} not found", id) }))
      ));
    };

  if let Some(name) = request_contact.name {
    contact.name = Set(name);
  }

  if let Some(email) = request_contact.email {
    contact.email = Set(email);
  }

  if let Some(phone) = request_contact.phone {
    contact.phone = Set(phone);
  }

  contact.updated_at = Set(Some(DateTime::from(NaiveDateTime::new(Utc::now().naive_utc().date(), Utc::now().naive_utc().time()))));

  let result = Contacts::update(contact)
    .filter(contacts::Column::Id.eq(id))
    .exec(&database)
    .await
    .map_err(|e| {
      eprintln!("Failed to update contact: {}", e);
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "Failed to update contact" }))
      )
    })?;

  Ok(Json(serde_json::json!({
    "id": id,
    "name": result.name,
    "email": result.email,
    "phone": result.phone,
    "created_at": result.created_at,
    "updated_at": result.updated_at
  })))
  
}