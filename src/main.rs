use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware};
use env_logger::Env;
use chrono::{Utc, Duration, Local};
use std::fmt::Write;

pub mod routes;
mod handlers;
mod models;
use crate::routes::{auction_routes};

#[get("/")]
async fn index() -> impl Responder {
    let now = chrono::Local::now();
    let time_string = now.format("%H:%M:%S").to_string();
    let duration = Duration::seconds(14400);
    let end_time = format_duration(duration);
    let end_time_string = end_time.to_string();


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
                    <h1 id="clock-display">{}</h1>
                </div>
            </body>
        </html>
    "#,
        time_string,
        end_time_string
    );

    HttpResponse::Ok().body(html)
}

fn format_duration(duration: Duration) -> String {
    let mut result = String::new();
    let seconds = duration.num_seconds();
    let minutes = seconds / 60;
    let hours = minutes / 60;

    write!(
        result,
        "{:02}:{:02}:{:02}",
        hours % 24,
        minutes % 60,
        seconds % 60
    ).unwrap();

    result
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

   HttpServer::new(|| {
       App::new()
            .wrap(middleware::Logger::default())
            .configure(auction_route::config)
            .service(index)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
