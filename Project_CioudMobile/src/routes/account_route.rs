use actix_web::web;
use crate::handlers::account_handler::{check_account};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(check_account);
}