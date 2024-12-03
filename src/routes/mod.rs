pub mod shorten;
pub mod redirect;
use actix_web::web;


pub fn init(cfg: &mut web::ServiceConfig) {
    shorten::init(cfg);
    redirect::init(cfg);
}