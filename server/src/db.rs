use rusqlite::Connection;

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
        ready_at TEXT,
        FOREIGN_KEY (item_id) REFERENCES menu (id),
    );
"#;

const ITEMS_MCDONALDS: &str = r#"
    INSERT INTO menu (name, prep_min_m, prep_max_m) VALUES
        ('Big Mac', 2.0, 3.0),
        ('Quarter Pounder with Cheese', 2.5, 4.0),
        ('Cheeseburger', 1.0, 2.0),
        ('McChicken', 2.0, 3.0),
        ('Filet-O-Fish', 2.5, 4.0),
        ('Chicken McNuggets (10 pieces)', 3.0, 5.0),
        ('French Fries (Medium)', 1.5, 2.5),
        ('French Fries (Large)', 1.5, 2.5),
        ('McFlurry', 1.0, 2.0),
        ('Apple Pie', 1.5, 2.0),
        ('Egg McMuffin', 2.0, 3.5),
        ('Sausage McMuffin', 2.0, 3.5),
        ('Bacon, Egg & Cheese Biscuit', 2.0, 3.5),
        ('Iced Coffee', 1.0, 1.5),
        ('McCafe Latte', 2.0, 3.0);
"#;

pub struct MenuRow {
    id: u64,
    name: String,
    prep_min_m: f32,
    prep_max_m: f32,
}

pub struct OrderRow {
    id: u64,
    table_id: u64,
    item_id: u64,
    ready_at: String,
}

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
