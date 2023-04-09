use actix_web::web;
use crate::handlers::account_handler::{check_account} ;

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/account/{id}")
            .route(web::get().to(check_account))
    );
}