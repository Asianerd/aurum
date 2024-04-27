use std::{collections::HashMap, sync::Mutex};

use rocket::State;

use crate::user::User;

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
// #endregion
