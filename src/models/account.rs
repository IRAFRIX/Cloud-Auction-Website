use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]

pub struct account {
    pub userId: i32,
    pub userName: String,
    pub userSurname: String,
    pub userPassword: i32,

}