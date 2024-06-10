use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    pub id: i32,
    pub abbreviation: String,
    pub city: String,
    pub conference: String,
    pub division: String,
    pub full_name: String,
    pub name: String,
}
