use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use redis::AsyncCommands;
use crate::models::url::{UrlRequest, UrlResponse};
use crate::utils::generate_short_url;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/shorten")
            .route(web::post().to(shorten_url))
    );
}
async fn shorten_url(
    redis_client: web::Data<Arc<tokio::sync::Mutex<redis::Client>>>,
    url_req: web::Json<UrlRequest>,
) -> impl Responder {
    let short_url = generate_short_url();

    let mut con = match redis_client.lock().await.get_multiplexed_async_connection().await {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match con.set::<String, String, ()>((&short_url).to_string(), (&url_req.original_url).to_string()).await {
        Ok(_) => HttpResponse::Ok().json(UrlResponse {
            short_url: format!("http://127.0.0.1:1025/api/{}", short_url),
        }),
        Err(_) => HttpResponse::InternalServerError().json("Failed to save short URL in Redis."),
    }
}