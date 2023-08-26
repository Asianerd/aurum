use std::sync::Mutex;
use actix_web::{web, App, HttpRequest, HttpServer};

#[derive(Clone, Debug)]
struct DataHolder {
    pub posts: Vec<post::Post>
}

mod post;

async fn index(req: HttpRequest) -> &'static str {
    println!("request : {req:?}");
    "can you understand me?"
}

async fn fetch_post(params: web::Path<(i64,)>, data: web::Data<DataHolder>) -> String {
    for p in &data.posts {
        if p.id == params.0 {
            return p.fetch_as_json();
        }
    }
    "no result".to_string()
}

async fn add_post(params: web::Path<(i64, String, String, Vec<String>)>, data: web::Data<Mutex<DataHolder>>) -> String {
    let mut d = data.lock().unwrap();
    d.posts.push(post::Post {
        id:params.0,
        title:params.1.clone(),
        description:params.2.clone(),
        images:params.3.to_owned()
    }.clone());
    println!("{:?}", d);
    "".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(Mutex::new(
                    DataHolder {
                        posts: vec![
                            post::Post {
                                id:0,
                                title:"lorem ipsum dolor sit amet".to_string(),
                                description:"this is the description".to_string(),
                                images:vec!["1.png".to_string()]
                            }
                        ]
                    }.clone()
                )).clone()
            )
            .route("/", web::get().to(index)) 
            .route("/fetch_post/{id}", web::get().to(fetch_post))
            .route("/add_post/{id}/{title}/{description}/{image}", web::get().to(add_post))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
