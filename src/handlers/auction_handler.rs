use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use log::{info, debug};
use serde_json::json;
use chrono::{Utc, Duration, Local};
use std::fmt::Write;

use crate::models::item::item;



#[get("/auctions")]
async fn get_all_auction_item(auction_item: web::Path<i32>) -> impl Responder {
    info!("get all item");

    let now = chrono::Local::now();

    let auction_item = vec![
        item {
            id: 112,
            name: "fan".to_string(),
            category: "electronics".to_string(),
            start_price: 300,
            remaining_time: format_duration(Duration::seconds(3600)),
            createdAt: now
        },
        item {
            id: 113,
            name: "fanta".to_string(),
            category: "electronics".to_string(),
            start_price: 30,
            remaining_time: format_duration(Duration::seconds(14400)),
            createdAt: now
        }
    ];

    let response_body = json!(auction_item);
  
    HttpResponse::Ok().json(response_body)
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