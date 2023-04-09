use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use log::{info, debug};
use serde_json::json;
use chrono::{Utc, Duration, Local};
use std::fmt::Write;

use crate::models::item::item;
use crate::models::u_item::uitem;



#[get("/auctions")]
async fn get_all_auction_item() -> impl Responder {
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
            category: "beverage".to_string(),
            start_price: 30,
            remaining_time: format_duration(Duration::seconds(14400)),
            createdAt: now
        }
    ];

    let response_body = json!(auction_item);
  
    HttpResponse::Ok().json(response_body)
}



#[post("/auctions/item")]
async fn create_item(item: web::Json<item>) -> impl Responder {
    info!("create new item");

    let auction_item = vec![
        item {
            name: item.name.clone(),
            category: item.category.clone(),
            start_price: item.start_price,
            remaining_time: item.remaining_time.clone(),
            id: item.id,
            createdAt: item.createdAt.clone(),
        }
    ];

    let response_body = json!(auction_item);
  
    HttpResponse::Created().json(response_body)
}

#[put("/auctions/items")]
async fn update_auction(uitem: web::Json<uitem>) -> impl Responder {
    info!("update item");
    let now = chrono::Local::now();

    let update_item = vec![
        uitem {
            name: uitem.name.clone(),
            category: uitem.category.clone(),
            start_price: uitem.start_price,
            remaining_time: uitem.remaining_time.clone(),
        },
        ];
        
        let response_body = json!(update_item);
        HttpResponse::Ok().json((response_body,"Successfully Updated",now))
}

#[delete("/auctions/items/{id}")]
async fn delete_auction_by_id(id: web::Path<i32>) -> impl Responder {
    let auction_id = id.into_inner();

    if auction_id == 114 {
        let response_body = json!({
            "message": "Successfully Deleted"
        });
        HttpResponse::Ok().json(response_body)
    }else{
        return HttpResponse::NotFound().json(json!({
            "message": "Not Found"
        }));
    }

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