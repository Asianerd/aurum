use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use rand::prelude::*;

use rocket::http::Status;
use rocket::request::Request;
use rocket::data::{self, Data, FromData};
use rocket::outcome::Outcome;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::account_handler::AccountHandler;
use crate::{log, utils};
use crate::wallet::{Wallet, WalletLimit, WalletResult};

const USER_ID_MAX: u128 = 4294967296u128; // 16^8, 2^32
const USER_ID_LENGTH: usize = 8usize; // number of letters for the code
// so that when represented in hex, itll be an 8 letter code

// XXXX-XXXX <- using this instead, 4 billion users

// XXXX-XXXX-XXXX
// or maybe XXX-XXX-XXX-XXX more readable? 
// TODO: revisit in future

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u128,
    pub username: String,
    pub wallets: HashMap<u128, Wallet>
}
impl User {
    pub fn save(account_handler: &AccountHandler) {
        fs::write("data/users.json", serde_json::to_string_pretty(&account_handler.users).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, User> {
        serde_json::from_str(fs::read_to_string("data/users.json").unwrap().as_str()).unwrap()
    }

    pub fn username_exists(account_handler: &AccountHandler, username: &String) -> bool {
        for (_, user) in &account_handler.users {
            if user.username == *username {
                return true;
            } 
        }
        false
    }

    pub fn generate_user_id(account_handler: &AccountHandler) -> u128 {
        let fallback = account_handler.users
            .keys()
            .max()
            .map_or(0, |i| i + 1);

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            // try generate for 1k times, else, resort to fallback
            let candidate = rng.gen_range(0..USER_ID_MAX);
            if account_handler.users.contains_key(&candidate) {
                continue;
            }

            return candidate;
        }
        fallback
    }

    pub fn lookup_user_id(account_handler: &AccountHandler, username: &String) -> Option<u128> {
        if !User::username_exists(account_handler, username) {
            return None;
        }
        account_handler.users
            .iter()
            .filter(|(_, u)| u.username == *username)
            .map(|(id, _)| *id)
            .next()
    }

    // #region user IDs
    pub fn encode_id(user_id: &u128) -> String {
        format!("{:0>8}", format!("{:X}", user_id).to_lowercase())
    }

    pub fn decode_id(code: &String) -> Option<u128> {
        if !User::is_valid_code(&code) {
            return None;
        }

        let code = User::sanitize_code(&code);
        Some(i64::from_str_radix(code.as_str(), 16).unwrap() as u128)
    }

    pub fn sanitize_code(code: &String) -> String {
        code.replace("#", "").replace("-", "")
    }

    pub fn is_valid_code(code: &String) -> bool {
        let code = User::sanitize_code(&code);
        // in case they give in #XXXX-XXXX
        if code.len() != USER_ID_LENGTH {
            return false;
        }
        // is able to be converted back to u128
        i64::from_str_radix(&code.as_str(), 16).is_ok()
    }
    // #endregion

    // #region querying
    pub fn query_username(account_handler: &AccountHandler, username: String) -> Vec<(u128, String, String)> { // id, 8 letter code, username
        // returns vector of users containing the username/id
        let mut result: Vec<(u128, String, String)> = vec![];
        for u in account_handler.users.values() {
            if result.len() >= 50 {
                // show only 20 results
                break;
            }
            if u.username.to_lowercase().contains(&(username.to_lowercase())) {
                result.push((
                    u.id.clone(),
                    User::encode_id(&u.id.clone()),
                    u.username.clone()
                ));
            }
        }

        if User::is_valid_code(&username) {
            let user_id = User::decode_id(&username).unwrap();

            if !account_handler.users.contains_key(&user_id) {
                return result;
            }

            if !result.iter().map(|(i, _, _)| i.clone()).any(|i| i == user_id) {
                // does not contain the one with correct ID
                result.insert(0, (
                    user_id,
                    User::encode_id(&user_id),
                    account_handler.users.get(&user_id).unwrap().username.clone()
                ));
            } else {
                // bring the matching id to the top
                for (index, u) in result.iter().enumerate() {
                    if u.0 == user_id {
                        let a = result.remove(index);
                        result.insert(0, a);
                        break;
                    }
                }
            }
        }

        result
    }
    // #endregion

    // #region wallet
    pub fn wallet_exists(&self, id: &u128) -> bool {
        self.wallets.contains_key(&id)
    }

    pub fn alter_balance(&mut self, wallet_id: &u128, amount: &f64) -> WalletResult {
        if !self.wallet_exists(&wallet_id) {
            return WalletResult::WalletNoExist;
        }
        self.wallets.get_mut(&wallet_id).unwrap().alter_balance(*amount)
    }

    pub fn transfer_balance(db: &mut AccountHandler, to: u128, to_wallet_id: u128, from: u128, from_wallet_id: u128, amount: f64) -> Result<WalletResult, LoginResult> {
        // assumption that amount is positive
        let from_user = match db.users.get(&from) {
            Some(u) => u,
            None => return Err(LoginResult::UsernameNoExist)
        };
        let from_wallet = if from_user.wallet_exists(&from_wallet_id) { from_user.wallets.get(&from_wallet_id).unwrap() } else { return Ok(WalletResult::WalletNoExist) };
        let from_result = from_wallet.can_alter_balance(-amount);
        // println!("{:?}", from_result);
        if from_result != WalletResult::Success {
            return Ok(from_result);
        }

        let to_user = match db.users.get(&from) {
            Some(u) => u,
            None => return Err(LoginResult::UsernameNoExist)
        };
        let to_wallet = if to_user.wallet_exists(&to_wallet_id) { to_user.wallets.get(&to_wallet_id).unwrap() } else { return Ok(WalletResult::WalletNoExist) };
        let to_result = to_wallet.can_alter_balance(amount);
        // println!("{:?}", to_result);
        if to_result != WalletResult::Success {
            return Ok(to_result);
        }

        // guard clauses after guard clauses

        db.users.get_mut(&from).unwrap().alter_balance(&from_wallet_id, &((-amount).clone()));
        db.users.get_mut(&to).unwrap().alter_balance(&to_wallet_id, &amount);
        db.log.log(utils::get_time(), log::Species::Transfer, to, to_wallet_id, from, from_wallet_id, amount);
        db.save();

        Ok(WalletResult::Success)
    }

    pub fn get_balance(&self, wallet_id: &u128) -> Result<f64, WalletResult> {
        if !self.wallet_exists(wallet_id) {
            return Err(WalletResult::WalletNoExist);
        }
        Ok(self.wallets.get(&wallet_id).unwrap().get_balance())
    }

    pub fn create_wallet(&mut self, name: String, colour: u128, limit: WalletLimit) {
        let id = self.generate_wallet_id();
        self.wallets.insert(id, Wallet {
            id,
            name,
            balance: 0f64,
            colour,
            limit,
            expenditure: 0f64
        });
    }

    pub fn delete_wallet(&mut self, wallet_id: &u128) -> WalletResult {
        if !self.wallet_exists(wallet_id) {
            return WalletResult::WalletNoExist;
        }

        if *wallet_id == 0 {
            return WalletResult::WalletIsDefault;
        }

        let balance = self.get_balance(wallet_id).unwrap();
        self.alter_balance(&0, &balance);
        // move all money to the general wallet

        self.wallets.remove(&wallet_id);

        WalletResult::Success
    }

    fn generate_wallet_id(&self) -> u128 {
        self.wallets.iter().map(|(i, _)| i).max().map_or(0, |i| i + 1)
    }
    // #endregion
}
// #[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Species {
//     Customer,
//     Vendor
// }



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInformation {
    pub username: String,
    pub password: String
}
impl LoginInformation {
    // handles anything to do with password or logging in
    fn get_passwords() -> HashMap<u128, String> {
        serde_json::from_str(fs::read_to_string("data/passwords.json").unwrap().as_str()).unwrap()
    }

    fn add_password(user_id: u128, password: &String) {
        let mut p = LoginInformation::get_passwords();
        p.insert(user_id, password.clone());
        fs::write("data/passwords.json", serde_json::to_string_pretty(&p).unwrap()).unwrap();
    }

    pub fn login(&self, account_handler: &AccountHandler) -> LoginResult {
        match User::lookup_user_id(account_handler, &self.username) {
            Some(id) => match LoginInformation::get_passwords().get(&id) {
                Some(p) => if self.password == *p { LoginResult::Success(id) } else { LoginResult::PasswordWrong },
                None => LoginResult::PasswordNoExist
            },
            None => LoginResult::UsernameNoExist
        }
    }

    pub fn signup(&self, account_handler: &mut AccountHandler) -> LoginResult {
        if User::username_exists(account_handler, &self.username) {
            return LoginResult::UsernameTaken;
        }

        let id = User::generate_user_id(account_handler);
        account_handler.users.insert(id, User {
            id,
            username: self.username.clone(),
            wallets: HashMap::from([(0, Wallet::default_wallet())])
        });
        LoginInformation::add_password(id, &self.password);
        account_handler.save();

        LoginResult::Success(id)
    }
}

#[rocket::async_trait]
impl<'l> FromData<'l> for LoginInformation {
    type Error = LoginInfoParseError;

    async fn from_data(_req: &'l Request<'_>, mut data: Data<'l>) -> data::Outcome<'l, Self> {
        let result = data.peek(512).await.to_vec();

        if result.is_empty() {
            // return OutCome::Error
            return Outcome::Error((
                Status::Ok,
                LoginInfoParseError::Empty
            ))
        }
        
        let result = result.iter().map(|x| (x.clone()) as char).collect::<String>();
        let result: HashMap<String, String> = serde_json::from_str(result.as_str()).unwrap();

        Outcome::Success(LoginInformation {
            username: result.get("username").unwrap().clone(),
            password: result.get("password").unwrap().clone()
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LoginInfoParseError {
    Success,

    ParsingError,

    Empty
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LoginResult {
    Success(u128),
    UsernameNoExist,
    PasswordNoExist, // consistency issue

    PasswordWrong,

    UsernameTaken,
}



// #region api calls
#[post("/", data="<login>")]
pub fn login(db: &State<Mutex<AccountHandler>>, login: LoginInformation) -> String {
    let db = db.lock().unwrap();
    serde_json::to_string(&login.login(&db)).unwrap()
}

#[post("/", data="<login>")]
pub fn signup(db: &State<Mutex<AccountHandler>>, login: LoginInformation) -> String {
    let mut db = db.lock().unwrap();
    serde_json::to_string(&login.signup(&mut db)).unwrap()
}

// apply caching
// remove username from cache when username is changed
#[get("/<username>")]
pub fn get_user_id(db: &State<Mutex<AccountHandler>>, username: String) -> String {
    let db = db.lock().unwrap();
    let result = User::lookup_user_id(&db, &username);
    match result {
        Some(id) => utils::parse_response_to_string(Ok(id)),
        None => utils::parse_response_to_string(Err(result))
    }
}

#[post("/", data="<login>")]
pub fn get_code(db: &State<Mutex<AccountHandler>>, login: LoginInformation) -> String {
    let db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(user_id) => utils::parse_response_to_string(Ok(User::encode_id(&user_id))),
        _ => utils::parse_response_to_string(Err(result))
    }
}

#[post("/<query_string>", data="<login>")]
pub fn query_users(db: &State<Mutex<AccountHandler>>, login: LoginInformation, query_string: String) -> String {
    let db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(_) => utils::parse_response_to_string(Ok(User::query_username(&db, urlencoding::decode(&query_string).unwrap().to_string()))),
        _ => utils::parse_response_to_string(Err(result))
    }
}
// #endregion
