extern crate diesel;
extern crate macadam;

use self::diesel::prelude::*;
use self::macadam::*;
use std::env::args;

fn main() {
    use macadam::schema::receipts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(receipts.filter(name.like(pattern)))
        .execute(&connection)
        .expect("Error revoking receipts");

    println!("Deleted {} receipts", num_deleted);
}
