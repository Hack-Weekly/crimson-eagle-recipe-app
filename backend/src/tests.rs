use super::rocket;
use crate::controllers;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client
        .get(uri!(controllers::recipe_controller::recipe))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}
