use std::{path::Path, sync::Mutex};

#[macro_use] extern crate rocket;

mod utils;
mod cors;

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
    // let cert_path = "/etc/letsencrypt/live/ozmium.xyz/fullchain.pem";
    // let key_path = "/etc/letsencrypt/live/ozmium.xyz/privkey.pem";

    // // Check if files exist
    // if !Path::new(cert_path).exists() {
    //     eprintln!("Certificate file not found: {}", cert_path);
    //     std::process::exit(1);
    // }

    // if !Path::new(key_path).exists() {
    //     eprintln!("Key file not found: {}", key_path);
    //     std::process::exit(1);
    // }

    rocket::custom(rocket::config::Config::figment().merge(("port", 8000)))
        .manage(Mutex::new(account_handler::AccountHandler::load()))
        .mount("/", routes![index])
        .mount("/save", routes![account_handler::save])
        .mount("/load", routes![account_handler::load])
        .mount("/generate_users", routes![account_handler::generate_users])
        .mount("/debug", routes![account_handler::debug])

        .mount("/login", routes![user::login])
        .mount("/signup", routes![user::signup])
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
        .mount("/wallet/get_limit", routes![wallet::get_limit])

        .attach(cors::CORS)
}