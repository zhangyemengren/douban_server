use std::env;
use reqwest;
use serde_json::Value;

pub async fn load(){
    let key = env::var("API_KEY").unwrap();
    let url = "https://api.balldontlie.io/v1/teams";
    let client = reqwest::Client::new();
    let res: Value = client.get(url)
        .header("Authorization", key).send().await.unwrap().json().await.unwrap();
    println!("{:?}", res);
}