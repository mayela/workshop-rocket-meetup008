#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> String {
    "It Works!".to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
