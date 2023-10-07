use std::fs;

use serde::{Serialize, Deserialize};
use rand::prelude::*;
use rand_distr::Normal;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stock {
    pub id: u64,
    pub name: String,

    pub magnitude: f64,

    pub history: Vec<f64>,
    // 0 -> inf
    pub growth_rate: f64,

    pub bankrupt: bool
}
impl Stock {
    pub fn age(&mut self) {
        if self.bankrupt {
            return;
        }

        let mut rand = thread_rng();
        let n = Normal::new(0.0, 5.0).unwrap();
        let mut delta = rand.sample(n) * self.magnitude;

        if self.history.len() >= 1 {
            delta += self.history[0];
        }
        self.history.insert(0, delta);
        self.growth_rate = self.growth_rate();

        self.bankrupt = self.history[0] < 0.0;
    }

    pub fn generate_svg(&self, width: f64, height: f64) {
        let line_container = (
            width * 0.1,
            height * 0.1,
            width * 0.8,
            height * 0.8
        );
        let line_area = (
            line_container.0,
            line_container.1 + (0.1 * line_container.3),
            0.85 * line_container.2,
            0.8 * line_container.3
        );

        let mut history = self.history[0..(self.history.len().clamp(0, 20))].to_vec();
        history.reverse();

        let rect_data = if history.len() >= 1 { fetch_rect_data(history, line_area) } else { "".to_string() };
        
        let result = format!("
        <svg width='{width}' height='{height}' viewBox='0 0 {width} {height}' fill='none' xmlns='http://www.w3.org/2000/svg'>
            <style>
                .regular {{
                    font: normal {}px sans-serif;
                }}
            </style>

            <defs>
                <marker id='arrow' viewBox='0 0 10 10' refX='5' refY='5' markerWidth='6' markerHeight='6' orient='auto-start-reverse'>
                    <path fill='#fff' d='M 0 0 L 10 5 L 0 10 z' />
                </marker>
            </defs>

            <rect width='{width}' height='{height}' fill='#24283b'/>

            <line x1='{}' y1='{}' x2='{}' y2='{}' stroke='#fff' stroke-width='2' marker-end='url(#arrow)'/>
            <line x1='{}' y1='{}' x2='{}' y2='{}' stroke='#fff' stroke-width='2' marker-end='url(#arrow)'/>

            {rect_data}
        </svg>",
            0.023 * width,

            // <rect x='{}' y='{}' width='{}' height='{}' stroke='#f00'/>
            // <rect x='{}' y='{}' width='{}' height='{}' stroke='#00f'/>

            // line_container.0, line_container.1, line_container.2, line_container.3,
            // line_area.0, line_area.1, line_area.2, line_area.3,


            // <text dominant-baseline='middle' text-anchor='middle' class='regular' x='{}' y='{}' transform='rotate(270 {}, {})' fill='#fff'>{} Inc. share sell price (USD)</text>

            // 0.07 * width, 0.5 * height,
            // 0.07 * width, 0.5 * height,
            // self.name,

            line_container.0, line_container.1 + line_container.3, line_container.0, line_container.1,
            line_container.0, line_container.1 + line_container.3, line_container.0 + line_container.2, line_container.1 + line_container.3
        ).to_string();

        fs::write(format!("aurum_output/{}.svg", self.name), result.trim()).expect("Unable to write file");

        fn fetch_rect_data(history: Vec<f64>, line_area: (f64, f64, f64, f64)) -> String {
            let mut result = "".to_string();

            // converting from f64 to i32 has data loss
            // inflate the data, change types, then deflate to negate the loss
            let highest = (history.iter().map(|x| (*x * 1000.0) as i64).max().unwrap() as f64) / 1000.0;
            let lowest = (history.iter().map(|x| (*x * 1000.0) as i64).min().unwrap() as f64) / 1000.0;

            let mut previous = 0.0;

            for (index, h) in history.iter().enumerate() {
                if index == 0 {
                    previous = *h;
                    continue;
                }
                let mut bullish = false;

                let mut bar_height = ((*h - previous) / (highest - lowest)) * line_area.3;
                if bar_height.abs() < 1.0 {
                    bar_height = bar_height.signum();
                }
                let mut bar_width = (((line_area.2 as f64) / (history.len() as f64)) as f64) * 0.5;
                let border_radius = bar_width * 0.1;
                
                let mut y_pos = line_area.1 + ((1.0 - ((h - lowest) / (highest - lowest))) * line_area.3);

                if (y_pos < 0.0) && (highest > 0.0) {
                    println!("{y_pos}\t{h}\t{highest}");
                }

                if bar_height > 0.0 {
                    // increasing bar
                    // current value greater than previous value
                    bullish = true;
                } else {
                    y_pos += bar_height;
                    bar_height = bar_height.abs();
                }
                // line_data += (format!("{} {} {} ",
                //     if index == 0 { "M" } else { "L" },
                //     line_area.0 + (((index as f64) / (history.len() as f64)) * line_area.2),
                //     line_area.1 + (((h - lowest) / (highest - lowest)) * line_area.3)
                // )).as_str();
                result += format!("<rect x='{:.2}' y='{y_pos:.2}' width='{bar_width:.2}' height='{bar_height:.2}' rx='{border_radius}' fill='#{}' />",
                    (line_area.0 + (((index as f64) / (history.len() as f64)) * line_area.2)) - (bar_width / 2.0),
                    if bullish { "0f0" } else { "f00" }
                ).as_str();

                y_pos -= bar_height / 2.0;
                y_pos += (bar_height / 4.0) * (if bullish { -1.0 } else { 1.0 });
                bar_height *= 2.0;
                bar_width *= 0.2;

                result += format!("<rect x='{:.2}' y='{y_pos:.2}' width='{bar_width:.2}' height='{bar_height:.2}' rx='{border_radius}' fill='#{}' />\n",
                    (line_area.0 + (((index as f64) / (history.len() as f64)) * line_area.2)) - (bar_width / 2.0),
                    if bullish { "0f0" } else { "f00" }
                ).as_str();

                previous = *h;
            }

            result
        }
    }

    pub fn growth_rate(&self) -> f64 {
        return ({
            if self.history.len() >= 2 {
                (self.history[0] / self.history[1]) * (
                    if (self.history[0].signum() + self.history[1].signum()) == 2.0 { 1.0 } else { -1.0 }
                    // edge cases when either number is negative
                )
            } else {
                0.0
            }
        } - 1.0) * 100.0;
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }


    pub fn fetch_stocks(stock_type: String, collection: &Vec<Stock>, amount: i32) -> String {
        let mut c = collection.clone();
        c.retain(|x| !x.bankrupt);

        let trend = match stock_type.as_str() {
            "trending" => StockTypes::Trending,
            "highest_value" => StockTypes::HighestValue,
            "highest_growth" => StockTypes::HighestGrowth,
            "lowest_value" => StockTypes::LowestValue,
            "lowest_growth" => StockTypes::LowestGrowth,
            _ => StockTypes::Invalid
        };

        match trend {
            StockTypes::Trending => {
                c.sort_by(|a, b| (a.growth_rate()).partial_cmp(&b.growth_rate()).unwrap());
            },
            StockTypes::HighestValue => {
                c.sort_by(|a, b| (b.history[0]).partial_cmp(&a.history[0]).unwrap());
            },
            StockTypes::HighestGrowth => {
                c.sort_by(|a, b| (b.growth_rate()).partial_cmp(&a.growth_rate()).unwrap());
            },
            StockTypes::LowestValue => {
                c.sort_by(|b, a| (b.history[0]).partial_cmp(&a.history[0]).unwrap());
            },
            StockTypes::LowestGrowth => {
                c.sort_by(|b, a| (b.growth_rate()).partial_cmp(&a.growth_rate()).unwrap());
            },
            StockTypes::Invalid => {
                return "none".to_string();
            }
        }
        // c
        //     [0..((c.len() as i32).clamp(0, amount) as usize)].to_vec()
        //     .iter()
        //     .map(|x| format!("{}\n", x.as_json()))
        //     .collect::<String>().to_string()
        serde_json::to_string(&c[0..((c.len() as i32).clamp(0, amount) as usize)].to_vec()).unwrap()
    }
}

enum StockTypes {
    Invalid,

    Trending,
    HighestValue,
    HighestGrowth,

    LowestValue,
    LowestGrowth
}
