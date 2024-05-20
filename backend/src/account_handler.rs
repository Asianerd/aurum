use std::{collections::HashMap, sync::Mutex};

use rand::rngs::ThreadRng;
use rocket::State;

use crate::{user::{self, User}, utils, wallet};

pub struct AccountHandler {
    pub users: HashMap<u128, User>
}
impl AccountHandler {
    pub fn new() -> AccountHandler {
        AccountHandler {
            users: HashMap::new()
        }
    }

    pub fn save(&self) {
        User::save(&self);
    }

    pub fn load() -> AccountHandler {
        let mut r = AccountHandler::new();
        r.users = User::load();

        r
    }
}

// #region api calls
#[get("/")]
pub fn load(db: &State<Mutex<AccountHandler>>) -> String {
    let mut db = db.lock().unwrap();
    *db = AccountHandler::load();
    "success".to_string()
}

#[get("/")]
pub fn save(db: &State<Mutex<AccountHandler>>) -> String {
    let db = db.lock().unwrap();
    db.save();
    "success".to_string()
}

#[get("/<number>")]
pub fn generate_users(db: &State<Mutex<AccountHandler>>, number: usize) -> String {
    let mut db = db.lock().unwrap();
    for _ in 0..number {
        let id = user::User::generate_user_id(&db);
        db.users.insert(id, user::User {
            id,
            username: utils::generate_name(&mut rand::thread_rng()),
            wallets: HashMap::from([(0, wallet::Wallet::default_wallet())])
        });
    }
    "success".to_string()
}

#[get("/")]
pub fn update_limits(db: &State<Mutex<AccountHandler>>) -> String {
    // reset expenditure at end of limit period
    // daily -> 12am utc
    // weekly -> monday utc
    // monthly -> 1st of month utc

    // ran once a day

    let mut db = db.lock().unwrap();

    // logic here

    "success".to_string()
}

#[get("/")]
pub fn debug(db: &State<Mutex<AccountHandler>>) -> String {
    let db = db.lock().unwrap();
    serde_json::to_string_pretty(&db.users).unwrap()
}
// #endregion
