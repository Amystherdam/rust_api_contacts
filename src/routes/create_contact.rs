use serde::{Deserialize, Serialize};
use validator::Validate;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use axum::{
  async_trait, 
  body::HttpBody, 
  extract::{rejection::JsonRejection, FromRequest}, 
  http::{Request, StatusCode}, BoxError, Json, RequestExt, Extension
};

use crate::database::contacts;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RequestContact {
  pub name: String,
  pub email: String,
  pub phone: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestContact
where
  B: HttpBody + Send + 'static,
  B::Data: Send,
  B::Error: Into<BoxError>,
  S: Send + Sync, // Exemplo de restrição para S
{
  type Rejection = (StatusCode, String);

  async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
    let Json(request_contact) = req
      .extract::<Json<RequestContact>, _>() // Provide both generic arguments
      .await
      .map_err(|error: JsonRejection| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

    Ok(request_contact)
  }
}

pub async fn create_contact(
  Extension(database): Extension<DatabaseConnection>, 
  Json(request_contact): Json<RequestContact>
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
  let new_contact = contacts::ActiveModel {
    name: Set(request_contact.name),    
    email: Set(request_contact.email),   
    phone: Set(request_contact.phone),
    ..Default::default()
  };

  let result = new_contact.save(&database).await.map_err(|e| {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to save contact: {}", e),
    )
  })?;

  Ok(Json(serde_json::json!({
    "id": result.id.unwrap(),
    "name": result.name.unwrap(),
    "email": result.email.unwrap(),
    "phone": result.phone.unwrap(),
  })))
}
