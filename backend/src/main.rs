use std::sync::Mutex;

#[macro_use] extern crate rocket;

mod utils;
mod cors;

mod account_handler;
mod user;
mod pin_handler;

#[get("/")]
pub fn index() -> String {
    "can you understand me?".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8000)))
        .manage(Mutex::new(account_handler::AccountHandler::load()))
        .mount("/", routes![index])

        .mount("/login", routes![user::login])
        .mount("/signup", routes![user::signup])

        .mount("/verify_pin", routes![pin_handler::api_verify_pin])

        .attach(cors::CORS)
}