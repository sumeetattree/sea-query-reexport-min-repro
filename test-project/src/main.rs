use common::sqlx;

sea_query::sea_query_driver_postgres!();
use sea_query_driver_postgres::bind_query;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
