use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::{self, mysql::MySqlPoolOptions};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    id: i32,
    abbreviation: String,
    city: String,
    conference: String,
    division: String,
    full_name: String,
    name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    data: Vec<Team>,
}

async fn load() -> Data {
    let key = env::var("API_KEY").unwrap();
    let url = "https://api.balldontlie.io/v1/teams";
    let client = reqwest::Client::new();
    let res: Data = client
        .get(url)
        .header("Authorization", key)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{:?}", res);
    res
}
async fn insert_data(data: Data) {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:qwer1234@localhost/nba-data")
        .await
        .unwrap();
    for team in data.data {
        sqlx::query!(
            r#"
            INSERT INTO team (id, abbreviation, city, conference, division, full_name, name)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            team.id,
            team.abbreviation,
            team.city,
            team.conference,
            team.division,
            team.full_name,
            team.name
        )
        .execute(&pool)
        .await
        .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let data = load().await;
    insert_data(data).await;
}
