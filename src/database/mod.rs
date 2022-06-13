use std::path::PathBuf;

use rusqlite::{params, Connection, Error, Params, Result, Row, Statement};

pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn new(file: &str) -> Self {
        let path = PathBuf::from(file);

        if path.exists() {
            return Self { path };
        }

        let db = Database { path };
        let file = std::fs::read_to_string("schema.sql").unwrap();
        db.execute_batch(&file);

        db
    }

    pub fn read_single<T: DbItem>(&self, sql: &str, params: impl Params) -> T {
        self.get_connection()
            .query_row(sql, params, move |r| Ok(T::from(r)))
            .expect("No single result was found")
    }

    pub fn read<T: DbItem>(&self, sql: &str, params: impl Params) -> Vec<T> {
        let conn = self.get_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut rows = stmt.query(params).unwrap();

        let mut items = vec![];
        while let Some(row) = rows.next().unwrap() {
            items.push(T::from(row))
        }

        items
    }

    pub fn write(&self, sql: &str, params: impl Params) {
        self.get_connection()
            .execute(sql, params)
            .expect("Could not execute SQL query");
    }

    fn execute(&self, sql: &str) {
        self.get_connection()
            .execute(sql, [])
            .expect("Could not execute SQL statement");
    }

    fn execute_batch(&self, sql: &str) {
        self.get_connection()
            .execute_batch(sql)
            .expect("Could not execute batch SQL statement");
    }

    fn insert<P: Params>(&self, sql: &str, params: P) {
        self.get_connection()
            .execute(sql, params)
            .expect("Could not execute SQL statement");
    }

    fn get_connection(&self) -> Connection {
        Connection::open(&self.path).expect("Could not open or create database")
    }
}

pub trait DbItem {
    fn from(row: &Row) -> Self;
}
