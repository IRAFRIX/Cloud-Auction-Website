use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

use serde_json::json;
use crate::models::user::account;

#[get("/account/{account_id}")]
async fn check_account(account_id: web::Path<String>) -> impl Responder {

        let userId: String = "630305123456789".to_string();
        let userName: String = "user".to_string();
        let userSurname: String = "user".to_string();

    let response_body = json!({
        "userName":*userName, 
        "userSurname" : userSurname,
    });
    if account_id == userId.into() {
        HttpResponse::Ok().header("Location", format!("/account/{}", account_id))
                          .json(response_body)
    } else {
        HttpResponse::Unauthorized().header("Location", format!("/account/{}", account_id))
                                     .json(json!({"message": "Unauthorized"}))
    }
}
