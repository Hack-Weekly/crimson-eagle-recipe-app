use rocket::Rocket;
use rocket::Build;
#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello])
}
