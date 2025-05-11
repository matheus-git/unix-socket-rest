use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Get(String),
    Post(Person),
    Delete(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Ok(Person),
    NotFound(String),
    Created,
    Deleted,
    Error(String),
}
