use actix_web::web;
use crate::handlers::auction_handler::{get_all_auction_item} ;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all_auction_item);
}