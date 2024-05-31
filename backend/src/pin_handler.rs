use std::{collections::HashMap, fs};
use std::sync::Mutex;

use rocket::State;
use serde::{Deserialize, Serialize};

use crate::utils;
use crate::{account_handler::AccountHandler, user::{LoginInformation, LoginResult}};

pub fn verify_pin(account_handler: &mut AccountHandler, login: &LoginInformation, pin: u8) -> Result<PinResult, LoginResult> {
    let result = login.login(account_handler);
    match result {
        LoginResult::Success(id) => {
            let pins: HashMap<u128, u8> = serde_json::from_str(fs::read_to_string("data/pins.json").unwrap().as_str()).unwrap();
            match pins.get(&id) {
                Some(i) => Ok(if *i == pin { PinResult::Success } else { PinResult::WrongPin }),
                None => Ok(PinResult::PinNoExist)
            }
        },
        _ => Err(result)
    }
}

pub fn register_pin(account_handler: &mut AccountHandler, login: &LoginInformation, pin: u8) -> Result<PinResult, LoginResult> {
    let result = login.login(account_handler);
    match result {
        LoginResult::Success(id) => {
            let mut pins: HashMap<u128, u8> = serde_json::from_str(fs::read_to_string("data/pins.json").unwrap().as_str()).unwrap();
            pins.insert(id,pin);
            Ok(PinResult::Success)
        },
        _ => Err(result)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PinResult {
    Success,
    WrongPin,
    PinNoExist, // ask to make pin

    UserIDExists,
}

// #region api calls
#[post("/<pin>", data="<login>")]
pub fn api_register_pin(db: &State<Mutex<AccountHandler>>, login: LoginInformation, pin: u8) -> String {
    let mut db = db.lock().unwrap();
    let result = register_pin(&mut db, &login, pin);
    match result {
        Ok(r) => utils::parse_response(Ok(serde_json::to_string(&r).unwrap())),
        Err(e) => utils::parse_response(Err(serde_json::to_string(&e).unwrap()))
    }
}

#[post("/<pin>", data="<login>")]
pub fn api_verify_pin(db: &State<Mutex<AccountHandler>>, login: LoginInformation, pin: u8) -> String {
    let mut db = db.lock().unwrap();
    let result = verify_pin(&mut db, &login, pin);
    match result {
        Ok(r) => match r {
            PinResult::Success => utils::parse_response(Ok(serde_json::to_string(&r).unwrap())),
            _ => utils::parse_response(Err(serde_json::to_string(&r).unwrap()))
        },
        Err(e) => utils::parse_response(Err(serde_json::to_string(&e).unwrap()))
    }
}
// #endregion
