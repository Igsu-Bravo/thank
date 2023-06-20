#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "<h1>Hello, World!</h1>"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
