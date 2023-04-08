use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize)]
pub struct item {
    pub id: i32,
    pub massage:String,
    pub name: String,
    pub category: String,
    pub start_price: i32,
    pub remaining_time: String,
    pub updatedAt: DateTime<Local>,
}