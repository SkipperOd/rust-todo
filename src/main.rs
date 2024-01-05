use rocket::{
    http::Status,
    serde::json::{json, Value},
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn hello() -> String {
    String::from("form simple string")
}

#[get("/")]
fn status() -> Status {
    Status::NotExtended
}

#[get("/")]
fn json_response() -> Value {
    let var_name = json!({
    "note": "it works"
    });
    var_name
}

#[launch]
fn rocket() -> _ {
    println!("starting rocket server!!, by default port is 8000",);
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
        .mount("/status", routes![status])
        .mount("/json", routes![json_response])
}
