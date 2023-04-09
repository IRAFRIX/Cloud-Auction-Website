use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

use serde_json::json;
use crate::models::user::account;

#[get("/account/{account_id}")]
async fn check_account(account_id: web::Path<String>) -> impl Responder {
    let account = account {
        userId : "630305123456789".to_string(),
        userName : "user".to_string(),
        userSurname : "user".to_string() ,
        
    };
    let response_body = json!(*account_id);
    if account_id == account.userId.into() {
        HttpResponse::Ok().header("Location", format!("/account/{}", account_id))
                          .json(response_body)
    } else {
        HttpResponse::Unauthorized().header("Location", format!("/account/{}", account_id))
                                     .json(json!({"message": "Unauthorized"}))
    }
}
