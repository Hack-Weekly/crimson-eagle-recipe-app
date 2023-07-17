#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use std::env;

use rocket::http::Method;
use rocket::{Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

mod controllers;
use controllers::{
    bookmark_controller, recipe_controller, recipe_create_controller, recipe_update_controller,
    tag_controller, user_controller,
};

mod apidoc;
mod database;
mod jwt;
mod models;
mod schema;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

    let allowed_origins = AllowedOrigins::some_exact(&[
        frontend_url,
        String::from("http://localhost:3000"),
        String::from("http://127.0.0.1:3000"),
        String::from("http://0.0.0.0:3000"),
    ]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS failed.");

    rocket::build()
        .mount(
            "/",
            routes![
                bookmark_controller::bookmarked_list,
                bookmark_controller::toggle_bookmark,
                recipe_controller::recipe,
                recipe_controller::search,
                recipe_controller::single_recipe,
                recipe_create_controller::create_recipe,
                recipe_update_controller::update_recipe,
                recipe_controller::delete,
                tag_controller::tag_list,
                tag_controller::single_tag,
                tag_controller::create_tag,
                tag_controller::toggle_tag,
                user_controller::login,
                user_controller::register,
                user_controller::profile,
                user_controller::change_password,
                apidoc::serve_api_doc,
            ],
        )
        .attach(cors)
}
