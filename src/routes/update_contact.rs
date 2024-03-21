use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde::Deserialize;
use crate::database::contacts::{self, Entity as Contacts};
use sea_orm::{entity::prelude::DateTime, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use chrono::{NaiveDateTime, Utc};


#[derive(Deserialize)]
pub struct RequestContact {
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
  )]
  pub name: Option<String>,
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
  )]
  pub email: Option<String>,
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
  )]
  pub phone: Option<String>,
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
  )]
  pub updated_at: Option<DateTime>
}

pub async fn update_contact(
  Path(id): Path<i32>,
  Extension(database): Extension<DatabaseConnection>,
  Json(request_contact): Json<RequestContact>
) -> Result<(), StatusCode> {
  let mut contact = 
    if let Some(contact) = Contacts::find_by_id(id)
      .one(&database)
      .await
      .map_err(|e| {
        eprintln!("Failed to fetch contact: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR})? 
    {
      contact.into_active_model()
    } else {
      return Err(StatusCode::NOT_FOUND);
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

  Contacts::update(contact)
    .filter(contacts::Column::Id.eq(id))
    .exec(&database)
    .await
    .map_err(|e| {
      eprintln!("Failed to update contact: {}", e);
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

  Ok(())
}