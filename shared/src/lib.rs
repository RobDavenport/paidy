use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: u64,
    pub name: String,
    pub prep_min_secs: f32,
    pub prep_max_secs: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemsRequest {
    pub items: Vec<u64>,
    pub table_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableResponse {
    pub table_id: u64,
    pub ordered_items: Vec<TableOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableOrder {
    pub table_id: u64,
    pub item_id: u64,
    pub finished_at: String,
}
