use axum::{
  extract::Query, http::StatusCode, Extension, Json
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Serialize, Deserialize};
use crate::database::contacts::{self, Entity as Contacts};

#[derive(Serialize)]
pub struct ResponseContact {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub phone: String,
  pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct Filters {
  name: Option<String>,
}

pub async fn read_contacts(
  Extension(database): Extension<DatabaseConnection>,
  Query(filters): Query<Filters>
) -> Result<Json<Vec<ResponseContact>>, StatusCode> {
  let mut name_filter = Condition::all();
  if let Some(name) = filters.name {
      name_filter = name_filter.add(contacts::Column::Name.contains(name));
  }

  let contacts = Contacts::find()
      .filter(name_filter)
      .all(&database)
      .await
      .map_err(|e| {
          (
              StatusCode::INTERNAL_SERVER_ERROR,
              format!("Failed to fetch contacts: {}", e),
          )
      })
      .unwrap()
      .iter()
      .map(|contact| ResponseContact {
          id: contact.id,
          name: contact.name.clone(),
          email: contact.email.clone(),
          phone: contact.phone.clone(),
          updated_at: contact.updated_at.clone(),
      })
      .collect();

  Ok(Json(contacts))
}