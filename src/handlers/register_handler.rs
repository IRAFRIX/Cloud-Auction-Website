use actix_web::{web, post,Responder, HttpResponse};
use serde_json::json;
use crate::models::{account::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct register_request {
    account: account,

}
#[post("/register")]
async fn post_register(user_data:web::Json<register_request>) -> impl Responder {
    let req = user_data.into_inner();

    let userName = req.account.userName;
    let userSurname = req.account.userSurname;
    let userId = req.account.userId;
    let userPassword = req.account.userPassword;
    

    HttpResponse::Ok().finish()
   

}


