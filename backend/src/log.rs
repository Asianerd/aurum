use std::{collections::HashMap, fs, sync::Mutex};

use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{account_handler::AccountHandler, user::{self, LoginInformation, LoginResult}, utils};

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct Entry {
    // pub id: u128,
    // pub species: Species,

    // pub time: u128, // epoch unix utc

    // pub to: u128,
    // pub to_wallet: u128,

    // // from and from_wallet are 0 when the transfer type is a topup
    // pub from: u128,
    // pub from_wallet: u128,

    // pub amount: f64,


    pub id: u128,
    pub species: Species,

    pub time: u128, // epoch unix utc

    pub to: u128,
    pub to_code: String,
    pub to_wallet: u128,
    pub to_name: String,
    pub to_wallet_name: String,

    pub from: u128,
    pub from_code: String,
    pub from_wallet: u128,
    pub from_name: String,
    pub from_wallet_name: String,

    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub entries: HashMap<u128, Entry>
    // TODO: some log caching system
    // append log ids where a user is attributed with to the user itself
    // user_id -> vec<log_id>
    // TODO: use vec instead of hashmap
}
impl Log {
    pub fn new() -> Log {
        Log {
            entries: HashMap::new()
        }
    }

    pub fn load() -> Log {
        // Log::new()
        serde_json::from_str(fs::read_to_string("data/logs.json").unwrap().as_str()).unwrap()
    }

    pub fn save(&self) {
        fs::write("data/logs.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }

    fn generate_log_id(&self) -> u128 {
        self.entries
            .keys()
            .max()
            .map_or(0, |i| i + 1)
    }

    pub fn get_log(&self, user_id: u128, from: u128) -> Vec<Entry> {
        let mut result: Vec<Entry> = vec![];

        for (_, e) in &self.entries {
            if e.time < from {
                continue;
            }
            if (e.to == user_id) || (e.from == user_id) {
                result.push(e.clone());
            }
        }

        result.sort_by_key(|e| e.time);
        result.reverse();

        result
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DetailedEntry {
    // replaces id with names and etc
    pub id: u128,
    pub species: Species,

    pub time: u128, // epoch unix utc

    pub to: u128,
    pub to_code: String,
    pub to_wallet: u128,
    pub to_name: String,
    pub to_wallet_name: String,

    pub from: u128,
    pub from_code: String,
    pub from_wallet: u128,
    pub from_name: String,
    pub from_wallet_name: String,

    pub amount: f64,
}
impl DetailedEntry {
    pub fn convert(db: &AccountHandler, e: &Entry) -> DetailedEntry {
        let to = db.users.get(&e.to);
        let from = db.users.get(&e.from);
        DetailedEntry {
            id: e.id,
            species: e.species.clone(),

            time: e.time,

            to: e.to,
            to_code: user::User::encode_id(&e.to),
            to_wallet: e.to_wallet,
            to_name: to.map_or("".to_string(), |u| u.username.clone()),
            to_wallet_name: to.map_or("".to_string(), |u| u.wallets.get(&e.to_wallet).map_or("".to_string(), |w| w.name.clone())),

            from: e.from,
            from_code: user::User::encode_id(&e.from),
            from_wallet: e.from_wallet,
            from_name: from.map_or("".to_string(), |u| u.username.clone()),
            from_wallet_name: from.map_or("".to_string(), |u| u.wallets.get(&e.from_wallet).map_or("".to_string(), |w| w.name.clone())),

            amount: e.amount
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Species {
    Transfer,
    TopUp,
}


pub fn log(db: &mut AccountHandler, time: u128, species: Species, to: u128, to_wallet: u128, from: u128, from_wallet: u128, amount: f64) {
    let id = db.log.generate_log_id();

    let to_user = db.users.get(&to);
    let from_user = db.users.get(&from);
    db.log.entries.insert(id, Entry {
        id,
        species,

        time,

        to,
        to_code: user::User::encode_id(&to),
        to_wallet,
        to_name: to_user.map_or("".to_string(), |u| u.username.to_string()),
        to_wallet_name: to_user
            .map_or(
                "".to_string(), 
                |u| u.wallets
                    .get(&to_wallet)
                    .map_or("".to_string(),|w| w.name.to_string()
                )
            ),

        from,
        from_code: user::User::encode_id(&from),
        from_wallet,
        from_name: from_user.map_or("".to_string(), |u| u.username.to_string()),
        from_wallet_name: from_user
            .map_or(
                "".to_string(), 
                |u| u.wallets
                    .get(&from_wallet)
                    .map_or("".to_string(),|w| w.name.to_string()
                )
            ),

        amount
    });
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TempHolder {
    pub entries: HashMap<u128, DetailedEntry>,
}

// #region api calls
#[get("/")]
pub fn debug_logs(db: &State<Mutex<AccountHandler>>) -> String {
    format!("{:?}", db.lock().unwrap().log)
}

// stating from when, in epoch unix
#[post("/<from>", data="<login>")]
pub fn get_logs(db: &State<Mutex<AccountHandler>>, login: LoginInformation, from: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => utils::parse_response_to_string(Ok(
            db
                .log
                .get_log(user_id, from)
                // .iter()
                // .map(|e| DetailedEntry::convert(&db, e))
                // .collect::<Vec<DetailedEntry>>()
        )),
        _ => utils::parse_response_to_string(Err(result))
    }
}
// #endregion
