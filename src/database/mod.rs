// database.rs
use redis::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init() -> Arc<Mutex<Client>> {
    let client = Client::open("redis://127.0.0.1/").expect("Invalid Redis URL");
    Arc::new(Mutex::new(client))
}