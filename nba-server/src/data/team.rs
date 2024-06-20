use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Conference {
    East,
    West,
}

impl Display for Conference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Conference::East => "east".to_string(),
            Conference::West => "west".to_string(),
        };
        write!(f, "{}", str)
    }
}
