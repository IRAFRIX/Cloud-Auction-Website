use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

use serde_json::json;
use crate::models::account::account;

#[get("/account/{account_id}")]
async fn check_account(account_id: web::Path<String>) -> impl Responder {
    let account = account {
        userId : "630305123456789".to_string(),
        userPassword : "1234".to_string(),
        userName : "user".to_string(),
        userSurname : "user".to_string() ,
        
    };
    // Check if account_id is valid and matches account.userId
    if account_id == account.userId {
        HttpResponse::Ok().header("Location", format!("/account/{}", account_id))
                          .json(account)
    } else {
        HttpResponse::Unauthorized().header("Location", format!("/account/{}", account_id))
                                     .json(json!({"message": "Unauthorized"}))
    }
}
