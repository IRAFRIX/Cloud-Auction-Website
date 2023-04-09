use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct account {
    pub userId: String,
    pub userName: String,
    pub userSurname: String
}