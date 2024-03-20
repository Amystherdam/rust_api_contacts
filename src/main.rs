use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use rust_api_contacts::run;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let database_uri: &str = dotenv!("DATABASE_URL");
    run(database_uri).await;
}
