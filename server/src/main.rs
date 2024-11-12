use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use rusqlite::Connection;
use shared::{Menu, MenuItem, OrderItemsRequest, TableOrder, TableResponse};
use tokio::sync::Mutex;

mod db;

#[derive(Clone)]
struct ServiceState {
    conn: Arc<Mutex<Connection>>,
}

#[tokio::main]
async fn main() {
    // Setup the service state
    let state = ServiceState {
        conn: Arc::new(Mutex::new(db::init_db())),
    };

    let service = Router::new()
        .route("/menu", get(get_menu))
        .route("/table/:table_id", get(get_table))
        .route(
            "/table/:table_id/:order_id",
            get(get_table_item).delete(delete_table_item),
        )
        .with_state(state);

    println!("Binding to 0.0.0.0:3030");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();

    println!("Service starting...");
    axum::serve(listener, service).await.unwrap();
}

type ServiceResponse<T> = Result<(StatusCode, T), HttpError>;

struct HttpError {
    status_code: StatusCode,
    body: String,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(self.status_code)
            .body(Body::new(self.body))
            .unwrap()
    }
}

/// Queries the database and returns the contents of the menu table. Generally called
/// at startup for each of the clients to populate their data.
async fn get_menu(State(state): State<ServiceState>) -> ServiceResponse<Json<Menu>> {
    const QUERY: &str = "SELECT * FROM menu;";
    let conn = state.conn.lock().await;

    let items = conn
        .prepare(QUERY)
        .unwrap()
        .query_map([], |row| {
            Ok(MenuItem {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                prep_min_secs: row.get(2).unwrap(),
                prep_max_secs: row.get(3).unwrap(),
            })
        })
        .unwrap()
        .flatten()
        .collect();

    Ok((StatusCode::OK, Json(Menu { items })))
}

// TODO:
// Client: query the items still remaining for a table.
// The application MUST, upon query request, show all items for a specified table number.
async fn get_table(
    State(state): State<ServiceState>,
    Path(table_id): Path<u64>,
) -> ServiceResponse<Json<TableResponse>> {
    Err(HttpError {
        body: "TODO".to_string(),
        status_code: StatusCode::NOT_IMPLEMENTED,
    })
}

// TODO:
// Client: add one or more items with a table number,
// The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
async fn post_table(
    State(state): State<ServiceState>,
    Path(table_id): Path<u64>,
    Json(payload): Json<OrderItemsRequest>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

// TODO:
// Client: remove an item for a table,
// The application MUST, upon deletion request, remove a specified item for a specified table number.
async fn delete_table_item(
    State(state): State<ServiceState>,
    Path((table_id, order_id)): Path<(u64, u64)>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

// TODO:
// Client: query a specific item remaining for a table
// The application MUST, upon query request, show a specified item for a specified table number.
async fn get_table_item(
    State(state): State<ServiceState>,
    Path((table_id, order_id)): Path<(u64, u64)>,
) -> ServiceResponse<Json<TableOrder>> {
    Err(HttpError {
        body: "TODO".to_string(),
        status_code: StatusCode::NOT_IMPLEMENTED,
    })
}
