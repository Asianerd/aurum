use std::sync::Mutex;

#[macro_use] extern crate rocket;

mod utils;
mod cors;

mod account_handler;
mod user;

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

        .attach(cors::CORS)
}