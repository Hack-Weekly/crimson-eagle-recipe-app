#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket::{Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

mod controller;
mod database;
mod models;
mod schema;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> Rocket<Build> {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "https://crimson-eagle-recipe-9002y03jg-crimson-eagle.vercel.app",
    ]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS failed.");

    rocket::build()
        .mount(
            "/",
            routes![
                controller::recipe,
                controller::search,
                controller::single_recipe,
                controller::addrecipes,
                controller::delete
            ],
        )
        .attach(cors)
}
