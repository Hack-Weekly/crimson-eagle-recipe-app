use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;
use crate::controller;

#[test]
fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(controller::recipe)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}
