#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    println!("starting rocket server!!, by default port is 8000",);
    rocket::build().mount("/", routes![index])
}
