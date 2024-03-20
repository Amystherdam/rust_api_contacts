mod database;
mod routes;

use sea_orm::Database;

pub async fn run(database_url: &str) {
  let database = Database::connect(database_url).await.unwrap();
  let app = routes::create_routes(database);

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}