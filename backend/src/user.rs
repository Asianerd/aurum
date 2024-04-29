use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use rocket::http::Status;
use rocket::request::Request;
use rocket::data::{self, Data, FromData};
use rocket::outcome::Outcome;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::account_handler::AccountHandler;
use crate::utils;
use crate::wallet::{Wallet, WalletLimit, WalletResult};

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
        account_handler.users
            .keys()
            .max()
            .map_or(0, |i| i + 1)
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
            limit
        });
    }

    pub fn delete_wallet(&mut self, wallet_id: &u128) -> WalletResult {
        if !self.wallet_exists(wallet_id) {
            return WalletResult::WalletNoExist;
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
            return Outcome::Failure((
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
// #endregion
