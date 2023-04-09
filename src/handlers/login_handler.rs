use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use serde_json::json;

/// POST method for `/login`
#[post("/login")]
async fn login(
    info: web::Json<LoginInfo>,  // expecting JSON payload containing `userId` and `userPassword`
) -> HttpResponse {
    // assuming `LoginInfo` struct with `userId` and `userPassword` fields
    let user_id = 630305123456789;
    let user_password = 1234;
    let user_surname = "user" ;

    // check if user credentials are correct
    let is_authenticated = authenticate(user_id, user_password);

    if is_authenticated {
        // assuming `User` struct with `user_id` and `user_surname` fields
        let user = get_user_info(user_id);
        let response_body = json!({ "message": format!("welcome {}", user.user_surname) });
        HttpResponse::Ok().json(response_body)
    } else {
        let response_body = json!({ "message": "userId or userPassword incorrect" });
        HttpResponse::Unauthorized().json(response_body)
    }
}
