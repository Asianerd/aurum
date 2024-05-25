use std::{str::FromStr, sync::Mutex};

use rocket::State;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::{account_handler::AccountHandler, log, user::{self, LoginInformation, LoginResult}, utils};

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    // per user id
    pub id: u128, // 0 -> main wallet
    pub name: String,
    pub balance: f64,
    pub colour: u128,

    pub limit: WalletLimit,
    // disable removing money if reached limit

    pub expenditure: f64, // total expenditure since start of limit
}
impl Wallet {
    pub fn default_wallet() -> Wallet {
        // returns empty wallet with id 0 and the name wallet
        Wallet {
            id: 0,
            name: "wallet".to_string(),
            balance: 0f64,
            colour: 0,
            limit: WalletLimit::Unlimited,
            expenditure: 0f64
        }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn can_alter_balance(&self, amount: f64) -> WalletResult {
        let mut n = self.balance.clone();
        n += amount;
        if n < 0f64 {
            return WalletResult::InsufficientAmount;
        }
        if (self.get_limit() != 0f64) && ((self.expenditure + amount.abs()) > self.get_limit()) {
            return WalletResult::ReachedLimit;
        }
        WalletResult::Success
    }

    pub fn alter_balance(&mut self, amount: f64) -> WalletResult {
        let mut n = self.balance.clone();
        n += amount;
        if n < 0f64 {
            return WalletResult::InsufficientAmount;
        }
        self.balance = n.clone();

        if amount < 0f64 {
            self.expenditure += amount.abs();
        }
        // TODO : implement ReachedLimit wallet result

        WalletResult::Success
    }

    pub fn get_limit(&self) -> f64 {
        match self.limit {
            WalletLimit::Daily(i) => i,
            WalletLimit::Weekly(i) => i,
            WalletLimit::Monthly(i) => i,        
            _ => 0f64
        }
    }

    pub fn get_limit_name(&self) -> String {
        // qol, idk how to use strum for this
        match &self.limit {
            WalletLimit::Daily(_) => "daily",
            WalletLimit::Weekly(_) => "weekly",
            WalletLimit::Monthly(_) => "monthly",
            _ => "unlimited"
        }.to_string()
    }

    //                              progress, limit
    pub fn get_limit_progress(&self) -> (f64, f64, String) {
        let limit = self.get_limit();
        (self.expenditure, limit, self.get_limit_name())
    }

    pub fn surpassed_limit(&self) -> bool {
        match self.limit {
            WalletLimit::Unlimited => false,
            _ => {
                let progress = self.get_limit_progress();
                progress.0 >= progress.1
            }
        }
    }

    pub fn update_wallet(&mut self, name: String, colour: u128, limit: WalletLimit) {
        if self.id != 0 {
            self.name = name.clone();
            self.limit = limit.clone();
        }
        self.colour = colour;
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WalletResult {
    Success,

    WalletNoExist,

    WalletIsDefault, // dont allow delete/updating

    InsufficientAmount,
    ReachedLimit
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
#[strum(serialize_all="lowercase")]
pub enum WalletLimit {
    Unlimited,
    Daily(f64), // timer resets at 0 GMT
    Weekly(f64), // timer reset every monday 0 gmt
    Monthly(f64) // timer resets every first of the month
}


// implement other currencies?

// #region api calls
#[post("/<name>/<colour>/<limit_type>/<limit>", data="<login>")]
pub fn create_wallet(db: &State<Mutex<AccountHandler>>, login: LoginInformation, name: String, colour: u128, limit_type: String, limit: f64) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            db.users.get_mut(&user_id).unwrap().create_wallet(name, colour, match WalletLimit::from_str(&limit_type) {
                Ok(t) => match t {
                    WalletLimit::Daily(_) => WalletLimit::Daily(limit),
                    WalletLimit::Weekly(_) => WalletLimit::Weekly(limit),
                    WalletLimit::Monthly(_) => WalletLimit::Monthly(limit),
                    _ => WalletLimit::Unlimited
                },
                Err(_) => WalletLimit::Unlimited
            });
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
        LoginResult::Success(user_id) => {
            let r = utils::parse_response_to_string(Ok(db.users.get_mut(&user_id).unwrap().delete_wallet(&wallet_id)));
            db.save();

            r
        },
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/<wallet_id>/<name>/<colour>/<limit_type>/<limit>", data="<login>")]
pub fn update_wallet(db: &State<Mutex<AccountHandler>>, login: LoginInformation, wallet_id: u128, name: String, colour: u128, limit_type: String, limit: f64) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            match db.users.get_mut(&user_id).unwrap().wallets.get_mut(&wallet_id) {
                Some(w) => {
                    w.update_wallet(name, colour, match WalletLimit::from_str(&limit_type) {
                        Ok(t) => match t {
                            WalletLimit::Daily(_) => WalletLimit::Daily(limit),
                            WalletLimit::Weekly(_) => WalletLimit::Weekly(limit),
                            WalletLimit::Monthly(_) => WalletLimit::Monthly(limit),
                            _ => WalletLimit::Unlimited
                        },
                        Err(_) => WalletLimit::Unlimited
                    });
                    db.save();
                    utils::parse_response_to_string(Ok("ok"))
                },
                None => utils::parse_response_to_string(Err(WalletResult::WalletNoExist))
            }
        },
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

// dk why i thought to make this an api call
// #[post("/<wallet_id>/<amount>", data="<login>")]
// pub fn alter_balance(db: &State<Mutex<AccountHandler>>, login: LoginInformation, wallet_id: u128, amount: f64) -> String {
//     let mut db = db.lock().unwrap();
//     let result = login.login(&db);
//     match result {
//         LoginResult::Success(user_id) => {
//             let r = utils::parse_response_to_string(Ok(db.users.get_mut(&user_id).unwrap().alter_balance(&wallet_id, &amount)));
//             db.save();
//             r
//         },
//         _ => utils::parse_response_to_string(Err(&result))
//     }
// }

#[post("/<wallet_id>/<amount>", data="<login>")]
pub fn top_up(db: &State<Mutex<AccountHandler>>, login: LoginInformation, wallet_id: u128, amount: f64) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            let r = utils::parse_response_to_string(Ok(db.users.get_mut(&user_id).unwrap().alter_balance(&wallet_id, &amount)));
            db.log.log(utils::get_time(), log::Species::TopUp, user_id, wallet_id, 0, 0, amount);
            db.save();
            r
        },
        _ => utils::parse_response_to_string(Err(&result))
    }
}

#[post("/<from_wallet>/<to_user>/<to_wallet>/<amount>", data="<login>")]
pub fn transfer_balance(db: &State<Mutex<AccountHandler>>, login: LoginInformation, from_wallet: u128, to_user: u128, to_wallet: u128, amount: f64) -> String {
    let mut db = db.lock().unwrap();
    let result =  login.login(&db);
    match result {
        LoginResult::Success(user_id) => utils::parse_response_to_string(Ok(user::User::transfer_balance(&mut db, to_user, to_wallet, user_id, from_wallet, amount))),
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/<from_wallet>/<to_wallet>/<amount>", data="<login>")]
pub fn transfer_between_wallets(db: &State<Mutex<AccountHandler>>, login: LoginInformation, from_wallet: u128, to_wallet: u128, amount: f64) -> String {
    let mut db = db.lock().unwrap();
    let result =  login.login(&db);
    match result {
        LoginResult::Success(user_id) => utils::parse_response_to_string(Ok(user::User::transfer_balance(&mut db, user_id, to_wallet, user_id, from_wallet, amount))),
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/<wallet_id>", data="<login>")]
pub fn get_limit(db: &State<Mutex<AccountHandler>>, login: LoginInformation, wallet_id: u128) -> String {
    let db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => {
            utils::parse_response_to_string(Ok(match db.users.get(&user_id).unwrap().wallets.get(&wallet_id) {
                Some(w) => w.get_limit_progress(),
                None => (0f64, 0f64, "Unlimited".to_string())
            }))
        },
        _ => utils::parse_response_to_string(Err(result))
    }
}
// #endregion
