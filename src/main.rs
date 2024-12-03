mod routes;
mod database;
mod models;
mod utils;

use actix_web::{web, App, HttpServer};
use crate::database::init as database_init;
use crate::routes::init as routes_init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_client = database_init().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .configure(routes_init)
    })
        .bind("127.0.0.1:1025")?
        .run()
        .await
}