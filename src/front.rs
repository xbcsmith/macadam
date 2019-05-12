#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

