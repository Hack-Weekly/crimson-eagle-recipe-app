#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod controller;
mod database;
mod models;
mod schema;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![
            controller::recipe,
            controller::search,
            controller::single_recipe,
            controller::addrecipes,
            controller::delete
        ],
    )
}
