use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use env_logger::Env;

pub mod routes;
mod handlers;
mod models;
use crate::routes::{auction_route,account_route,login_route,register_route};



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

   HttpServer::new(|| {
       App::new()
            .wrap(middleware::Logger::default())
            .configure(auction_route::config)
            .configure(account_route::config)
            .configure(login_route::config)
            .configure(register_route::config)

   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
