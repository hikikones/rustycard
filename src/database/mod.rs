use std::{path::Path, rc::Rc};

use rusqlite::{Connection, OpenFlags, Params, Row};

#[derive(Clone)]
pub struct Database {
    connection: Rc<Connection>,
}

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Self {
        match Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_WRITE) {
            Ok(conn) => Self {
                connection: Rc::new(conn),
            },
            Err(_) => {
                // Database not found. Create one.
                match Connection::open(&path) {
                    Ok(conn) => {
                        let schema = std::fs::read_to_string("schema.sql").unwrap();
                        conn.execute_batch(&schema).unwrap();
                        Self {
                            connection: Rc::new(conn),
                        }
                    }
                    Err(err) => {
                        panic!("Error opening database: {}", err)
                    }
                }
            }
        }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.read_many("SELECT * FROM cards", [])
    }

    pub fn create_card(&self, content: &str) -> usize {
        self.write_single("INSERT INTO cards (content) VALUES (?)", [content])
    }

    fn read_single<T: DbItem>(&self, sql: &str, params: impl Params) -> T {
        match self
            .connection
            .query_row(sql, params, move |row| Ok(T::from(row)))
        {
            Ok(item) => item,
            Err(err) => panic!("Error query row: {}", err),
        }
    }

    fn read_many<T: DbItem>(&self, sql: &str, params: impl Params) -> Vec<T> {
        let mut stmt = match self.connection.prepare(sql) {
            Ok(stmt) => stmt,
            Err(err) => panic!("Error preparing query: {}", err),
        };
        let rows = match stmt.query_map(params, |row| Ok(T::from(row))) {
            Ok(rows) => rows,
            Err(err) => panic!("Error query map: {}", err),
        };

        rows.filter_map(|item| item.ok()).collect()
    }

    fn write_single(&self, sql: &str, params: impl Params) -> usize {
        match self.connection.execute(sql, params) {
            Ok(id) => id,
            Err(err) => panic!("Error execute query: {}", err),
        }
    }

    fn write(&self, sql: &str, params: impl Params) {
        self.connection
            .execute(sql, params)
            .expect("Could not execute SQL query");
    }

    fn execute(&self, sql: &str) {
        self.connection
            .execute(sql, [])
            .expect("Could not execute SQL statement");
    }

    fn execute_batch(&self, sql: &str) {
        self.connection
            .execute_batch(sql)
            .expect("Could not execute batch SQL statement");
    }

    fn insert<P: Params>(&self, sql: &str, params: P) {
        self.connection
            .execute(sql, params)
            .expect("Could not execute SQL statement");
    }
}

pub trait DbItem {
    fn from(row: &Row) -> Self;
}

pub struct Card {
    pub id: usize,
    pub content: String,
    // TODO
}
impl DbItem for Card {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0).unwrap(),
            content: row.get(1).unwrap(),
        }
    }
}
