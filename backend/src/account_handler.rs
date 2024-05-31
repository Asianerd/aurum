use std::{collections::HashMap, sync::Mutex};

use rand::rngs::ThreadRng;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{log::Log, user::{self, User}, utils, wallet};

#[derive(Clone, Serialize, Deserialize)]
pub struct AccountHandler {
    pub users: HashMap<u128, User>,
    pub log: Log,
}
impl AccountHandler {
    pub fn new() -> AccountHandler {
        AccountHandler {
            users: HashMap::new(),
            log: Log::new()
        }
    }

    pub fn save(&self) {
        User::save(&self);
        self.log.save();
    }

    pub fn load() -> AccountHandler {
        let mut r = AccountHandler::new();
        r.users = User::load();
        r.log = Log::load();

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


    let reset_status: (bool, bool, bool) = (
        false, false, false
    );

    for (user_id, user) in &mut db.users {
        for (wallet_id, wallet) in &mut user.wallets {
            match wallet.limit {
                wallet::WalletLimit::Daily(_) => { if reset_status.0 { wallet.expenditure = 0f64; } }
                wallet::WalletLimit::Weekly(_) => { if reset_status.1 { wallet.expenditure = 0f64; } }
                wallet::WalletLimit::Monthly(_) => { if reset_status.2 { wallet.expenditure = 0f64; } }
                _ => {}
            }
        }
    }

    "success".to_string()
}

#[get("/")]
pub fn debug(db: &State<Mutex<AccountHandler>>) -> String {
    let db = db.lock().unwrap();
    serde_json::to_string_pretty(&db.users).unwrap()
}
// #endregion
