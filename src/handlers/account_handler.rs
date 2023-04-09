use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
fn get_account_by_id(account_id: web::Path<String>) -> impl Responder {
    let account = Account {
        let userId = 630305123456789;
        let userPassword = 1234;
        let userSurname = "user" ;
    };

    // Check if account_id is valid and matches account.userId
    if account_id == account.user_Id {
        HttpResponse::Ok().header("Location", format!("/account/{}", account_id))
                          .json(account)
    } else {
        HttpResponse::Unauthorized().header("Location", format!("/account/{}", account_id))
                                     .json(json!({"message": "Unauthorized"}))
    }
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/account/{id}")
            .route(web::get().to(get_account_by_id))
    );
}
