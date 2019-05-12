extern crate diesel;
extern crate macadam;

use self::diesel::prelude::*;
use self::macadam::models::*;
use self::macadam::*;

fn main() {
    use macadam::schema::receipts::dsl::*;

    let connection = establish_connection();
    let results = receipts
        .filter(issued.eq(true))
        .limit(5)
        .load::<Receipt>(&connection)
        .expect("Error loading receipts");

    println!("Displaying {} receipts", results.len());
    for receipt in results {
        println!("{}", receipt.name);
        println!("----------\n");
        println!("{}", receipt.status);
    }
}
