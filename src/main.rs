use std::sync::Mutex;
#[macro_use] extern crate rocket;
use rocket::{State, data};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod post;
mod user;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "cors headers",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Debug)]
pub struct DataHolder {
    pub posts: Vec<post::Post>,
    pub users: Vec<user::User>
}

#[get("/")]
fn index() -> &'static str {
    "can you understand me?"
}

#[get("/")]
fn fetch_all(data_holder: &State<Mutex<DataHolder>>) -> String {
    let data = data_holder.lock().unwrap();
    format!("{:?}", serde_json::to_string(&(*data).posts).unwrap().as_str())
}

#[get("/<title>/<description>/<image>")]
fn add_post(title: String, description: String, image: String, data_holder: &State<Mutex<DataHolder>>) -> String {
    let mut data = data_holder.lock().unwrap();
    let new_id = (*data).posts.len() as i64;
    (*data).posts.push(post::Post {
        id: new_id,
        title: title,
        description: description,
        images: vec![
            image
            ]
    });
    format!("{data_holder:?}")
}

#[get("/<id>")]
fn fetch_post(id: i64, data_holder: &State<Mutex<DataHolder>>) -> String {
    let data = data_holder.lock().unwrap();
    for p in &(*data).posts {
        if id == p.id {
            return p.fetch_as_json();
        }
    }
    "".to_string()
}


#[get("/")]
fn fetch_all_users(data_holder: &State<Mutex<DataHolder>>) {
    format!("{:?}", (*(data_holder.lock().unwrap())).users);
}

#[get("/<username>/<profile_pic>")]
fn add_user(username: String, profile_pic: String, data_holder: &State<Mutex<DataHolder>>) -> String {
    let mut data = data_holder.lock().unwrap();
    let new_id = (*data).users.len() as i64;
    (*data).users.push(user::User {
        id: new_id,
        username: username,
        profile_pic: profile_pic
    });
    "".to_string()
}

#[get("/<id>")]
fn fetch_user(id: i64, data_holder: &State<Mutex<DataHolder>>) -> String {
    for u in &(*(data_holder.lock().unwrap())).users {
        if id == u.id {
            return u.fetch_as_json();
        }
    }
    "".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(DataHolder {
            posts: vec![],
            users: vec![]
        }))
        .mount("/", routes![index])
        .mount("/fetch_all_posts", routes![fetch_all])
        .mount("/add_post", routes![add_post])
        .mount("/fetch_post", routes![fetch_post])

        .mount("/fetch_all_users", routes![fetch_all_users])
        .mount("/fetch_user", routes![fetch_user])
        .mount("/add_user", routes![add_user])
        .attach(CORS)
}
