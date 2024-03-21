// #[cfg(test)]
// mod tests {
//   use reqwest::Client;
//   use axum::{
//     body::Body, 
//     routing::{get, post, patch, delete}, 
//     Router
    
//   };
//   use axum::Server;
//   use sea_orm::Database;
//   // use core::net::SocketAddr;

//   //  use std::net::SocketAddr;

//   //  use std::os::unix::net::SocketAddr;

//    use tokio::net::unix::SocketAddr;

  use crate::routes::create_routes;

//   #[tokio::test]
//   async fn test_create_routes() {
//       let database = Database::connect("postgres://postgres:postgres@localhost:5432/rust_api_contact")
//           .await
//           .unwrap();

//       let app = create_routes(database);

//       // Crie uma instância do servidor Axum
//       let service = RouterService::new(app);
//       let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // Endereço para testes locais
//       let server = Server::from_tcp(service, &addr).unwrap();

//       // Inicie o servidor Axum em uma nova thread
//       let join_handle = tokio::spawn(async move {
//           server.await.unwrap();
//       });

//       // Faça solicitações para todas as rotas e verifique as respostas
//       let client = Client::new();
//       let response = client.get("http://localhost:3000/contacts").send().await.unwrap();
//       assert_eq!(response.status(), 200);

//       // Encerre o servidor
//       join_handle.await.unwrap();
//   }

// }
// // #[cfg(test)]
// // mod tests {
// //     use axum_test::TestServer;
// //     use axum::{
// //         body::Body,
// //         routing::{get, post, patch, delete},
// //         Router,
// //     };
// //     use sea_orm::Database;
// //     use crate::routes::{
// //         read_contact::read_contact,
// //         read_contacts::read_contacts,
// //         create_contact::create_contact,
// //         update_contact::update_contact,
// //         delete_contact::delete_contact
// //     };
// //     use crate::routes::create_routes;
// //     use axum::http::{StatusCode, Response};
// //     use axum::handler::{Handler, HandlerError};

// //     async fn test_handler() -> Result<Response<Body>, HandlerError> {
// //         Ok(Response::new(Body::from("Test successful")))
// //     }

// //     #[tokio::test]
// //     async fn test_create_routes() {
// //         let database = Database::connect("mysql://root:password@localhost:3306/contacts").await.unwrap();
// //         let app = create_routes(database);

// //         let client = TestServer::new(app);

// //         // Teste de rota GET /contacts
// //         let response = client.get("/contacts").send().await.unwrap();
// //         assert_eq!(response.status(), StatusCode::OK);

// //         // Teste de rota POST /contacts
// //         let response = client.post("/contacts").send().await.unwrap();
// //         assert_eq!(response.status(), StatusCode::OK);

// //         // Teste de rota GET /contacts/:id
// //         let response = client.get("/contacts/1").send().await.unwrap();
// //         assert_eq!(response.status(), StatusCode::OK);

// //         // Teste de rota PATCH /contacts/:id
// //         let response = client.patch("/contacts/1").send().await.unwrap();
// //         assert_eq!(response.status(), StatusCode::OK);

// //         // Teste de rota DELETE /contacts/:id
// //         let response = client.delete("/contacts/1").send().await.unwrap();
// //         assert_eq!(response.status(), StatusCode::OK);

// //         // Teste de uma rota de teste
// //         let response = client.get("/test").send().await.unwrap();
// //         assert_eq!(response.status(), StatusCode::OK);
// //         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
// //         assert_eq!(body, b"Test successful");
// //     }
// // }


#[cfg(test)]
use axum::{
    body::{Body},
    http::{StatusCode, Request}
    , Json} ;

use sea_orm::Database;

use tower::ServiceExt;


#[tokio::test]
async fn test_authorize() {
    let db = Database::connect("postgres://postgres:postgres@localhost:5432/rust_api_contact")
              .await
              .unwrap();
            let app = create_routes(db);
    let client_id = "dotenvy::var";
  let client_secret = "dotenvy::var().unwrap()";
    let request_body = format!("client_id={},client_secret={}", client_id, client_secret);
    let request = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::from(request_body))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}