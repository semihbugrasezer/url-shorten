use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UrlRequest {
    pub original_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct UrlResponse {
    pub short_url: String,
}