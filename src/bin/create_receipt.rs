extern crate diesel;
extern crate macadam;

use self::macadam::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Name : ");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!(
        "\nCreating {} (Press {} when finished)\n",
        name, EOF
    );
    let mut status = String::new();
    stdin().read_to_string(&mut status).unwrap();

    let _ = create_receipt(&connection, name, &status);
    println!("\nSaved draft {}", name);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
