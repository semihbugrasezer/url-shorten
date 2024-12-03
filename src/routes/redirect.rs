use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use redis::AsyncCommands;
use tokio::sync::Mutex;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/{short_url}", web::get().to(redirect_url));
}

async fn redirect_url(
    redis_client: web::Data<Arc<Mutex<redis::Client>>>,
    path: web::Path<String>,
) -> impl Responder {
    let short_url = path.into_inner();
    let mut con = match redis_client.lock().await.get_multiplexed_async_connection().await {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match con.get::<&str, String>(&short_url).await {
        Ok(original_url) => HttpResponse::Found()
            .append_header(("Location", original_url))
            .finish(),
        Err(_) => HttpResponse::NotFound().body("URL not found!"),
    }
}