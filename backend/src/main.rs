use std::{fs, path::Path, sync::Mutex};

#[macro_use] extern crate rocket;

mod utils;
mod cors;

mod soterius;

mod log;
mod wallet;
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
        .mount("/save", routes![account_handler::save])
        .mount("/load", routes![account_handler::load])
        .mount("/generate_users", routes![account_handler::generate_users])
        .mount("/debug", routes![account_handler::debug])

        .mount("/debug_log", routes![log::debug_logs])
        .mount("/log/fetch_logs", routes![log::get_logs])

        // .mount("/login", routes![user::login])
        // .mount("/signup", routes![user::signup])
        .mount("/lookup_username", routes![user::get_user_id])
        .mount("/query_users", routes![user::query_users])
        .mount("/get_code", routes![user::get_code])

        .mount("/verify_pin", routes![pin_handler::api_verify_pin])
        .mount("/register_pin", routes![pin_handler::api_register_pin])

        .mount("/wallet/get_wallets", routes![wallet::get_wallets])
        .mount("/wallet/get_balance", routes![wallet::get_balance])
        .mount("/wallet/get_total_balance", routes![wallet::get_total_balance])
        .mount("/wallet/top_up", routes![wallet::top_up])
        .mount("/wallet/create_wallet", routes![wallet::create_wallet])
        .mount("/wallet/transfer_balance", routes![wallet::transfer_balance])
        .mount("/wallet/transfer_between_wallets", routes![wallet::transfer_between_wallets])
        .mount("/wallet/get_limit", routes![wallet::get_limit])
        .mount("/wallet/update_wallet", routes![wallet::update_wallet])
        .mount("/wallet/delete_wallet", routes![wallet::delete_wallet])

        .attach(cors::CORS)
}