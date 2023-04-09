use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]

pub struct account {
    pub userId: String,
    pub userName: String,
    pub userSurname: String,
    pub userPassword: String,

}