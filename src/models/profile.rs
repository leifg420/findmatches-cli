mod models;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub preferences: Vec<String>,
}
