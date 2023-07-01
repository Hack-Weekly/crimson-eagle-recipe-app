use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}
