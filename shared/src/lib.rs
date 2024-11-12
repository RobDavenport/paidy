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
