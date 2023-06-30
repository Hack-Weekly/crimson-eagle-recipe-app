use rocket::Rocket;
use rocket::Build;
#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/recipe")]
fn recipe() -> &'static str {
    "Recipe page"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello, recipe])
}
