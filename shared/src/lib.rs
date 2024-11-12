use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItem {
    id: u64,
    name: String,
    price: f32,
    prep_min_secs: f32,
    prep_max_secs: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemsRequest {
    items: Vec<u64>,
    table_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableResponse {
    id: u64,
    ordered_items: Vec<TableOrder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableOrder {
    item_id: u64,
    finished_at: String,
}
