use std::env;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Team {
    id: i32,
    abbreviation: String,
    city: String,
    conference: String,
    division: String,
    full_name: String,
    name: String
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Data {
    data: Vec<Team>
}

pub fn load(){
    let key = env::var("API_KEY").unwrap();
    let url = "https://api.balldontlie.io/v1/teams";
    let client = reqwest::blocking::Client::new();
    let res: Data = client.get(url)
        .header("Authorization", key).send().unwrap().json().unwrap();
    println!("{:?}", res);
}

fn main() {
    load();
}