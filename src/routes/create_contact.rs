use serde::{Deserialize, Serialize};
use validator::Validate;
use sea_orm::{
  entity::prelude::DateTime, 
  ActiveModelTrait, 
  DatabaseConnection, 
  Set
};
use axum::{http::StatusCode, Json, Extension};
use chrono::{NaiveDateTime, Utc};
use crate::database::contacts;
use regex::Regex;

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestContact {
  #[validate(
    length(min = 3, max = 100, message = "Name must be between 3 and 100 characters"), 
    regex(path = Regex::new(r"^[a-zA-Z\s.']+$").unwrap(), message = "Name must only contain letters and spaces")
  )]
  pub name: String,

  #[validate(email(message = "Invalid email"))]
  pub email: String,

  #[validate(
    length(equal = 11, message = "Phone must be 11 characters"),
    regex(path = Regex::new(r"^\d{11}$").unwrap(), message = "Phone must only contain numbers")
  )]
  pub phone: String,
}

pub async fn create_contact(
  Extension(database): Extension<DatabaseConnection>, 
  Json(request_contact): Json<RequestContact>
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
  match request_contact.validate() {
    Ok(()) => {
      let new_contact = contacts::ActiveModel {
        name: Set(request_contact.name),    
        email: Set(request_contact.email),   
        phone: Set(request_contact.phone),
        created_at: Set(Some(DateTime::from(NaiveDateTime::new(Utc::now().naive_utc().date(), Utc::now().naive_utc().time())))),
        updated_at: Set(Some(DateTime::from(NaiveDateTime::new(Utc::now().naive_utc().date(), Utc::now().naive_utc().time())))),
        ..Default::default()
      };

      match new_contact.save(&database).await {
        Ok(result) => {
          Ok(Json(serde_json::json!({
            "id": result.id.unwrap(),
            "name": result.name.unwrap(),
            "email": result.email.unwrap(),
            "phone": result.phone.unwrap(),
            "created_at": result.created_at.unwrap(),
            "updated_at": result.updated_at.unwrap()
          })))
        }, 
        Err(errors) => {
          Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("Failed to create contact: {}", errors) })),
          ))
        }
      }
    }, 
    Err(errors) => {
      Err((
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({ "errors": errors })),
      ))
    }
  }
}
