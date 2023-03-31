use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware};
use env_logger::Env;
use chrono::Local;

pub mod routes;
mod handlers;
mod models;
use crate::routes::{cart_routes, shipping_routes};

#[get("/")]
async fn index() -> impl Responder {
    let now = chrono::Local::now();
    let time_string = now.format("%H:%M:%S").to_string();

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Web Clock</title>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="stylesheet" href="/static/style.css">
            </head>
            <body>
                <div class="clock">
                    <h1 id="clock-display">{}</h1>
                </div>
            </body>
        </html>
    "#,
        time_string
    );

    HttpResponse::Ok().body(html)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

   HttpServer::new(|| {
       App::new()
            .wrap(middleware::Logger::default())
            .configure(cart_routes::config)
            .configure(shipping_routes::config)
            .service(index)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
