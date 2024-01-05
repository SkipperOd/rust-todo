use rocket::{http::Status, serde::json::Json};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("hello from update api")))
}

#[launch]
fn rocket() -> _ {
    println!("starting rocket server!!, by default port is 8000",);
    rocket::build().mount("/", routes![index])
}
