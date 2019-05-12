extern crate diesel;
extern crate macadam;

use self::diesel::prelude::*;
use self::macadam::*;
use std::env::args;

fn main() {
    use macadam::schema::receipts::dsl::{receipts, issued};

    let id = args()
        .nth(1)
        .expect("issue_receipt requires a receipt id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::update(receipts.find(id))
        .set(issued.eq(true))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to find receipt {}", id));

    let receipt: models::Receipt = receipts
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find receipt {}", id));

    println!("Issued receipt {}", receipt.name);
}
