#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod database;
mod models;
mod schema;
mod controller;

#[cfg(test)] mod tests;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        controller::recipe, 
        controller::search,
        controller::addrecipes, 
        controller::delete])
}
