use actix_web::{web, post, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::models::account::account;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    userId: u64,
    userPassword: u64,
}

fn authenticate(userId: u64, userPassword: u64) -> bool {
    // implementation for authentication logic
    true
}

fn get_user_info(userId: u64) -> account {
    // implementation for retrieving user information from database
    account {
        userId: userId.to_string(),
        userPassword: "password".to_string(),
        userName: "User".to_string(),
        userSurname: "Name".to_string(),
    }
}

/// POST method for /login
#[post("/login")]
async fn login(
    info: web::Json<LoginInfo>,  // expecting JSON payload containing userId and userPassword
) -> HttpResponse {
    let userId = info.userId;
    let userPassword = info.userPassword;

    // check if user credentials are correct
    let is_authenticated = authenticate(userId, userPassword);

    if is_authenticated {
        // assuming User struct with user_id and user_surname fields
        let account = get_user_info(userId);
        let response_body = json!({ "message": format!("welcome {}", account.userSurname) });
        HttpResponse::Ok().json(response_body)
    } else {
        let response_body = json!({ "message": "userId or userPassword incorrect" });
        HttpResponse::Unauthorized().json(response_body)
    }
}