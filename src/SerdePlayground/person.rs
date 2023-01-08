use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

// Basic struct with no attributes
pub struct Person {
    pub name: String,
    pub age: u8,
    pub occupation: String,
    pub married: Married,
    pub phone_numbers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Married {
    YES,
    NO,
}
