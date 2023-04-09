use actix_web::{web, post,Responder, HttpResponse};
use serde_json::json;
use crate::models::{account::account};
use serde::{Serialize, Deserialize};


#[post("/register")]
async fn post_register(account:web::Json<account>) -> impl Responder {

    let auction_item = vec![
        account {
            userId: account.userId.clone(),
            userName: account.userName.clone(),
            userSurname: account.userSurname.clone(),
            userPassword: account.userPassword.clone(),
        }
    ];
    
    let response_body = json!({
        "message": "Account Created"
    });
    HttpResponse::Created().json(response_body)
   

}


