use std::sync::Mutex;

use rocket::State;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::{account_handler::AccountHandler, user::{LoginInformation, LoginResult}, utils};

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    // per user id
    pub id: u128, // 0 -> main wallet
    pub name: String,
    pub balance: f64,
    pub colour: u128,

    pub limit: WalletLimit
    // disable removing money if reached limit
}
impl Wallet {
    pub fn default_wallet() -> Wallet {
        // returns empty wallet with id 0 and the name wallet
        Wallet {
            id: 0,
            name: "wallet".to_string(),
            balance: 0f64,
            colour: 0,
            limit: WalletLimit::Unlimited
        }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn alter_balance(&mut self, amount: f64) -> WalletResult {
        let mut n = self.balance.clone();
        n += amount;
        if n < 0f64 {
            return WalletResult::InsufficientAmount;
        }
        self.balance = n;
        WalletResult::Success
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WalletResult {
    Success,

    WalletNoExist,

    InsufficientAmount
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
pub enum WalletLimit {
    Unlimited,
    Daily(f64),
    Weekly(f64),
    Monthly(f64)
}


// implement other currencies?

// #region api calls
#[post("/<name>/<colour>/<limit_type>/<limit>", data="<login>")]
pub fn create_wallet(db: &State<Mutex<AccountHandler>>, login: LoginInformation, name: String, colour: u128, limit_type: String, limit: f64) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            // TODO : finishe this
            db.users.get_mut(&user_id).unwrap().create_wallet(name, colour, (lower, upper));
            db.save();
            utils::parse_response_to_string(Ok("success"))
        },
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/<wallet_id>", data="<login>")]
pub fn delete_wallet(db: &State<Mutex<AccountHandler>>, login: LoginInformation, wallet_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => utils::parse_response_to_string(Ok(db.users.get_mut(&user_id).unwrap().delete_wallet(&wallet_id))),
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/", data="<login>")]
pub fn get_wallets(db: &State<Mutex<AccountHandler>>, login: LoginInformation) -> String {
    let db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            let mut w = db.users.get(&user_id).unwrap().wallets
                .values()
                .map(|w| w.clone())
                .collect::<Vec<Wallet>>();
            w.sort_by_key(|w| w.id);
            utils::parse_response_to_string(Ok(w))
        },
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/", data="<login>")]
pub fn get_total_balance(db: &State<Mutex<AccountHandler>>, login: LoginInformation) -> String {
    let db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => utils::parse_response_to_string(
            Ok(db.users.get(&user_id).unwrap().wallets
                .values()
                .map(|w| w.get_balance())
                .sum::<f64>()
            )
        ),
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/<wallet_id>", data="<login>")]
pub fn get_balance(db: &State<Mutex<AccountHandler>>, login: LoginInformation, wallet_id: u128) -> String {
    let db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            let r = utils::parse_response_to_string(Ok(db.users.get(&user_id).unwrap().get_balance(&wallet_id)));
            db.save();
            r
        },
        _ => utils::parse_response_to_string(Err(&result))
    }
}

#[post("/<wallet_id>/<amount>", data="<login>")]
pub fn alter_balance(db: &State<Mutex<AccountHandler>>, login: LoginInformation, wallet_id: u128, amount: f64) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            utils::parse_response_to_string(Ok(db.users.get_mut(&user_id).unwrap().alter_balance(&wallet_id, &amount)))

        },
        _ => utils::parse_response_to_string(Err(&result))
    }
}
// #endregion
