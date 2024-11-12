use std::convert::Infallible;

use warp::{http::StatusCode, Filter, Reply};

#[tokio::main]
async fn main() {
    // TODO: Setup the database
    // TODO: Fill the menu items with some basic menu items

    let menu_routes = warp::path("menu").and_then(get_menu);
    let table = warp::path!("table" / u64);

    let table_routes = table.and(warp::post()).and_then(add_items_to_table);

    warp::serve(menu_routes.or(table_routes))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn get_menu() -> Result<impl Reply, Infallible> {
    // TODO: Query the DB for the menu items
    // and return them
    Ok("TODO".to_string())
}

// TODO:
// Client: add one or more items with a table number,
// The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
async fn add_items_to_table(table_id: u64) -> Result<impl Reply, Infallible> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}

// TODO:
// Client: remove an item for a table,
// The application MUST, upon deletion request, remove a specified item for a specified table number.
async fn remove_item_from_table() -> Result<impl Reply, Infallible> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}

// TODO:
// Client: query the items still remaining for a table.
// The application MUST, upon query request, show all items for a specified table number.
async fn query_table() -> Result<impl Reply, Infallible> {
    Ok("TODO".to_string())
}

// TODO:
// Client: query a specific item remaining for a table
// The application MUST, upon query request, show a specified item for a specified table number.
async fn query_table_item() -> Result<impl Reply, Infallible> {
    Ok("TODO".to_string())
}
