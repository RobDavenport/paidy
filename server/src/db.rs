use std::{collections::HashMap, sync::Arc};

use axum::http::StatusCode;
use chrono::Duration;
use rusqlite::Connection;
use shared::{MenuItem, OrderItemsRequest, TableOrder};
use tokio::sync::Mutex;

use crate::HttpError;

const INIT_DB_QUERY: &str = r#"
    CREATE TABLE menu (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        prep_min_m REAL NOT NULL,
        prep_max_m REAL NOT NULL
    );

    CREATE TABLE orders (
        id INTEGER PRIMARY KEY,
        table_id INTEGER NOT NULL,
        item_id INTEGER NOT NULL,
        ready_at TEXT NOT NULL,
        FOREIGN_KEY (item_id) REFERENCES menu (id),
    );
"#;

const ITEMS_MCDONALDS: &str = r#"
    INSERT INTO menu (name, prep_min_m, prep_max_m) VALUES
        ('Big Mac', 6.0, 12.0),
        ('Quarter Pounder with Cheese', 5.0, 8.0),
        ('Cheeseburger', 5.0, 8.0),
        ('McChicken', 8.0, 12.0),
        ('Filet-O-Fish', 5.0, 8.0),
        ('Chicken McNuggets (10 pieces)', 6.0, 10.0),
        ('French Fries (Medium)', 5.0, 7.0),
        ('French Fries (Large)', 5.0, 7.0),
        ('McFlurry', 5.0, 15.0),
        ('Apple Pie', 5.0, 9.0),
        ('Egg McMuffin', 5.0, 7.0),
        ('Sausage McMuffin', 5.0, 7.0),
        ('Bacon, Egg & Cheese Biscuit', 5.0, 7.0),
        ('Iced Coffee', 5.0, 6.0),
        ('McCafe Latte', 5.0, 6.0);
"#;

/// Initializes a database and calls the initialization query
/// which fills it with some temporary data
pub fn init_db() -> Connection {
    // Note: In a real situation, would use an actual database somewhere
    // and persist data across sessions
    let conn = Connection::open_in_memory().unwrap();
    println!("Initialized connection to DB");

    conn.execute(INIT_DB_QUERY, ()).unwrap();
    println!("Setup tables successfully.");

    conn.execute(ITEMS_MCDONALDS, ()).unwrap();
    println!("Filled menu successfully.");

    conn
}

fn handle_query_error(error: rusqlite::Error) -> HttpError {
    use rusqlite::Error::*;

    let status_code = match error {
        SqliteFailure(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
        SqliteSingleThreadedMode => StatusCode::INTERNAL_SERVER_ERROR,
        FromSqlConversionFailure(_, _, _) => StatusCode::INTERNAL_SERVER_ERROR,
        IntegralValueOutOfRange(_, _) => StatusCode::BAD_REQUEST,
        Utf8Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        NulError(_) => StatusCode::BAD_REQUEST,
        InvalidParameterName(_) => StatusCode::BAD_REQUEST,
        InvalidPath(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ExecuteReturnedResults => StatusCode::INTERNAL_SERVER_ERROR,
        QueryReturnedNoRows => StatusCode::NOT_FOUND,
        InvalidColumnIndex(_) => StatusCode::INTERNAL_SERVER_ERROR,
        InvalidColumnName(_) => StatusCode::INTERNAL_SERVER_ERROR,
        InvalidColumnType(_, _, _) => StatusCode::INTERNAL_SERVER_ERROR,
        StatementChangedRows(_) => StatusCode::INTERNAL_SERVER_ERROR,
        ToSqlConversionFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        InvalidQuery => StatusCode::INTERNAL_SERVER_ERROR,
        UnwindingPanic => StatusCode::INTERNAL_SERVER_ERROR,
        MultipleStatement => StatusCode::BAD_REQUEST,
        InvalidParameterCount(_, _) => StatusCode::BAD_REQUEST,
        SqlInputError { .. } => StatusCode::BAD_REQUEST,
        InvalidDatabaseIndex(_) => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpError {
        status_code,
        body: error.to_string(),
    }
}

pub async fn get_menu(connection: &Arc<Mutex<Connection>>) -> Result<Vec<MenuItem>, HttpError> {
    const QUERY: &str = "SELECT * FROM menu;";

    Ok(connection
        .lock()
        .await
        .prepare(QUERY)
        .map_err(handle_query_error)?
        .query_map([], |row| {
            Ok(MenuItem {
                id: row.get(0)?,
                name: row.get(1)?,
                prep_min_secs: row.get(2)?,
                prep_max_secs: row.get(3)?,
            })
        })
        .map_err(handle_query_error)?
        .flatten()
        .collect())
}

pub async fn get_tables_items(
    connection: &Arc<Mutex<Connection>>,
    table_id: i64,
) -> Result<Vec<TableOrder>, HttpError> {
    const QUERY: &str = "SELECT * FROM orders WHERE table_id == ?1;";

    Ok(connection
        .lock()
        .await
        .prepare(QUERY)
        .map_err(handle_query_error)?
        .query_map([table_id], |row| {
            Ok(TableOrder {
                table_id,
                item_id: row.get(2)?,
                finished_at: row.get(3)?,
            })
        })
        .map_err(handle_query_error)?
        .flatten()
        .collect())
}

pub async fn order_items(
    connection: &Arc<Mutex<Connection>>,
    table_id: i64,
    items: OrderItemsRequest,
) -> Result<Vec<TableOrder>, HttpError> {
    let menu_items = menu_lookup(connection, &items.items).await?;

    // Item ID, PrepTime
    let rows: Vec<(i64, String)> = items
        .items
        .iter()
        .flat_map(|id| Some((*id, menu_items.get(id)?.get_random_prep_time())))
        .collect();

    todo!()
}

struct MenuItemRow {
    name: String,
    prep_min_m: f64,
    prep_max_m: f64,
}

impl MenuItemRow {
    fn get_random_prep_time(&self) -> String {
        let range = self.prep_max_m - self.prep_min_m;
        let mins = self.prep_min_m + fastrand::f64() * range;
        let secs = mins.round() as i64;

        return (chrono::Utc::now() + Duration::seconds(secs)).to_rfc3339();
    }
}

async fn menu_lookup(
    connection: &Arc<Mutex<Connection>>,
    item_ids: &[i64],
) -> Result<HashMap<i64, MenuItemRow>, HttpError> {
    let ids = item_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let query = format!("SELECT id, prep_min_m, prep_max_m FROM menu WHERE id IN ({ids})");

    Ok(connection
        .lock()
        .await
        .prepare(&query)
        .map_err(handle_query_error)?
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                MenuItemRow {
                    name: row.get(1)?,
                    prep_min_m: row.get(2)?,
                    prep_max_m: row.get(3)?,
                },
            ))
        })
        .map_err(handle_query_error)?
        .flatten()
        .collect())
}
