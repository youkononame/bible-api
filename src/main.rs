#[macro_use] extern crate rocket;

mod routes;
mod models;
use crate::routes::bible;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![bible::index, bible::get_book, bible::get_chapter, bible::get_verse])
}