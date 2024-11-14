use serde::{Deserialize, Serialize};

pub const SERVICE_URL: &str = "127.0.0.1:3030";

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: i64,
    pub name: String,
    pub prep_min_m: f32,
    pub prep_max_m: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemsRequest {
    pub items: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableResponse {
    pub table_id: i64,
    pub ordered_items: Vec<TableOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableOrder {
    pub order_id: i64,
    pub item_id: i64,
    pub ready_at: String,
}
