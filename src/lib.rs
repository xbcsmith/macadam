#[macro_use]
extern crate diesel;

extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::NewReceipt;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_receipt(conn: &SqliteConnection, name: &str, status: &str) -> usize {
    use schema::receipts;

    let new_receipt = NewReceipt { name, status };

    diesel::insert_into(receipts::table)
        .values(&new_receipt)
        .execute(conn)
        .expect("Error saving new receipt")
}
