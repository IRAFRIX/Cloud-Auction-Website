use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use serde_json::json;

/// POST method for `/login`
#[post("/login")]
async fn login(
    info: web::Json<String>,  // expecting JSON payload containing `userId` and `userPassword`
) -> HttpResponse {
    // assuming `LoginInfo` struct with `userId` and `userPassword` fields
    let userId = 630305123456789;
    let userPassword = 1234;
    let userSurname = "user" ;

    // check if user credentials are correct
    let is_authenticated = authenticate(userId, userPassword);

    if is_authenticated {
        // assuming `User` struct with `user_id` and `user_surname` fields
        let user = user_info(userId);
        let response_body = json!({ "message": format!("welcome {}", account.userSurname) });
        HttpResponse::Ok().json(response_body)
    } else {
        let response_body = json!({ "message": "userId or userPassword incorrect" });
        HttpResponse::Unauthorized().json(response_body)
    }
}
