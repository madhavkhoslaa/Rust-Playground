use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8, 
    pub occupation: String,
    pub married: Married
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Married {
    YES,
    NO
}