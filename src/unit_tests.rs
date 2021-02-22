use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_index() {
    let client =
        Client::new(rocket::ignite().mount("/", routes![super::index, super::ping])).expect("");
    assert_eq!(client.get("/").dispatch().status(), Status::Ok);
}

#[test]
fn test_ping() {
    let client =
        Client::new(rocket::ignite().mount("/", routes![super::index, super::ping])).expect("");
    assert_eq!(
        client.get("/ping").dispatch().body_string(),
        Some("pong".into())
    );
}

#[test]
#[should_panic]
fn test_fail() {
    let client =
        Client::new(rocket::ignite().mount("/", routes![super::index, super::ping])).expect("");
    assert_eq!(client.get("/pleaseFail").dispatch().status(), Status::Ok);
}
