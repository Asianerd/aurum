use std::sync::Mutex;
use std::time::Instant;
#[macro_use] extern crate rocket;
use rocket::State;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use rand::prelude::*;



mod stock;
use stock::Stock;

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
    pub stocks: Vec<stock::Stock>
}
impl DataHolder {
    fn new() -> DataHolder {
        let mut r = DataHolder {
            stocks: vec![]
        };

        // r.stocks.push(Stock {
        //    id: 0u64,
        //    name: "yttrium".to_string(),
        //    magnitude: 0.05f64,
        //    history: vec![200f64]
        // });

        for i in ["Hydrogen","Helium","Lithium","Beryllium","Boron","Carbon","Nitrogen","Oxygen","Fluorine","Neon","Sodium","Magnesium","Aluminum","Silicon","Phosphorus","Sulfur","Chlorine","Argon","Potassium","Calcium","Scandium","Titanium","Vanadium","Chromium","Manganese","Iron","Cobalt","Nickel","Copper","Zinc","Gallium","Germanium","Arsenic","Selenium","Bromine","Krypton","Rubidium","Strontium","Yttrium","Zirconium","Niobium","Molybdenum","Technetium","Ruthenium","Rhodium","Palladium","Silver","Cadmium","Indium","Tin","Antimony","Tellurium","Iodine","Xenon","Cesium","Barium","Lanthanum","Cerium","Praseodymium","Neodymium","Promethium","Samarium","Europium","Gadolinium","Terbium","Dysprosium","Holmium","Erbium","Thulium","Ytterbium","Lutetium","Hafnium","Tantalum","Wolfram","Rhenium","Osmium","Iridium","Platinum","Gold","Mercury","Thallium","Lead","Bismuth","Polonium","Astatine","Radon","Francium","Radium","Actinium","Thorium","Protactinium","Uranium","Neptunium","Plutonium","Americium","Curium","Berkelium","Californium","Einsteinium","Fermium","Mendelevium","Nobelium","Lawrencium","Rutherfordium","Dubnium","Seaborgium","Bohrium","Hassium","Meitnerium","Darmstadtium","Roentgenium","Copernicium","Nihonium","Flerovium","Moscovium","Livermorium","Tennessine","Oganesson"] {
        //for i in ["Carbon"] {
            r.stocks.push(Stock {
                id: r.stocks.len() as u64,
                name: i.to_string(),
                magnitude: rand::thread_rng().gen_range(0.01..=10.0),
                history: vec![rand::thread_rng().gen_range(250.0..350.0)],
                growth_rate: 0.0,
                bankrupt: false
            });
        }

        for _ in 0..100 {
            for x in &mut r.stocks {
                x.age();
            }
        }

        // r.stocks = vec![
        //     Stock {
        //         id: 0,
        //         name: "Child Grooming".to_string(),
        //         magnitude: 1000.0,
        //         history: vec![5000.0],
        //         growth_rate: 0.0
        //     }
        // ];

        for x in &mut r.stocks {
            x.generate_svg(1100.0, 700.0);
        }

        // r.stocks = vec![
        //     Stock {
        //         id: 0,
        //         name: "test".to_string(),
        //         magnitude: 50.0,
        //         history: vec![
        //             100.0, 120.0
        //         ],
        //         growth_rate:0.0
        //     }
        // ];
        // println!("Growth rate : {}", r.stocks[0].growth_rate());
        // panic!();
        r
    }
}


#[get("/")]
fn index() -> &'static str {
    "can you understand me?"
}

#[get("/<iterations>")]
fn age(iterations:i32, data_holder: &State<Mutex<DataHolder>>) -> String {
    let mut result = "".to_string();

    let mut data = data_holder.lock().unwrap();
    let now = Instant::now();
    for x in &mut (*data).stocks {
        for _ in 0..iterations {
            x.age();
        }
        result += &format!("{:?} : {:?} values, \n", x.name, x.history.len()).to_string();
    }

    format!("{} stocks aged\n{:?}μs elapsed ({}s)\n{result}", (*data).stocks.len(), now.elapsed().as_micros(), now.elapsed().as_secs_f64())
}

#[get("/")]
fn get_svg(data_holder: &State<Mutex<DataHolder>>) -> String {
    let data = data_holder.lock().unwrap();

    let now = Instant::now();
    for x in &(*data).stocks {
        x.generate_svg(1600f64, 900f64);
        println!("{}'s graph generated", x.name);
    }

    format!("{} svgs generated\n{:?}ms elapsed ({}s)", (*data).stocks.len(), now.elapsed().as_millis(), now.elapsed().as_secs_f64())
}

#[get("/")]
fn age_svg(data_holder: &State<Mutex<DataHolder>>) -> String {
    let mut result = "".to_string();

    let mut data = data_holder.lock().unwrap();
    let now = Instant::now();
    for x in &mut (*data).stocks {
        x.age();
        result += &format!("{:?} : {:?} values, \n", x.name, x.history.len()).to_string();
    }

    for x in &(*data).stocks {
        x.generate_svg(1600f64, 900f64);
        println!("{}'s graph generated", x.name);
    }

    format!("{} svgs generated\n{:?}ms elapsed ({}s)\n{} stocks aged\n{:?}μs elapsed ({}s)\n{result}", (*data).stocks.len(), now.elapsed().as_millis(), now.elapsed().as_secs_f64(), (*data).stocks.len(), now.elapsed().as_micros(), now.elapsed().as_secs_f64())

}

#[get("/<trend>/<amount>")]
fn fetch_stocks(amount: i32, trend: String, data_holder: &State<Mutex<DataHolder>>) -> String {
    let data = data_holder.lock().unwrap();
    stock::Stock::fetch_stocks(trend, &data.stocks, amount)
}

#[get("/")]
fn purge(data_holder: &State<Mutex<DataHolder>>) {
    let mut data = data_holder.lock().unwrap();
    for x in &mut data.stocks {
        x.history = vec![];
        x.growth_rate = 0.0;
        x.bankrupt = false;
    }
}

#[get("/")]
fn debug(data_holder: &State<Mutex<DataHolder>>) -> String {
    let data = data_holder.lock().unwrap();
    let mut result = "".to_string();
    for x in &data.stocks {
        result += format!("{} : {}\n", x.name, x.growth_rate).as_str();
    }
    result
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(DataHolder::new()))
        .mount("/", routes![index])
        .mount("/age", routes![age])
        .mount("/svg", routes![get_svg])
        .mount("/debug", routes![debug])
        .mount("/fetch_stock", routes![fetch_stocks])
        .mount("/age_svg", routes![age_svg])
        .mount("/purge", routes![purge])

        .attach(CORS)
}
