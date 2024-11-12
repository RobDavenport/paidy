use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

// TODO:
// Client: add one or more items with a table number,
// The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.

// TODO:
// Client: remove an item for a table,
// The application MUST, upon deletion request, remove a specified item for a specified table number.

// TODO:
// Client: query the items still remaining for a table.
// The application MUST, upon query request, show all items for a specified table number.

// TODO:
// Client: query a specific item remaining for a table
// The application MUST, upon query request, show a specified item for a specified table number.
