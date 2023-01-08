use std::vec;

use crate::SerdePlayground::{self, person::Person, Employee::Employee, Employee::Team};


pub fn run() {
    // let madhav = SerdePlayground::person::Person {
    //     name: String::from("Madhav Khosla"),
    //     age: 23,
    //     occupation: String::from("Engineer"),
    //     married: SerdePlayground::person::Married::NO,
    //     phone_numbers: vec![String::from("7678282691"), String::from("9958492713")]
    // };

    let madhav: Employee = Employee {
        name: String::from("Madhav Khosla"),
        age: 24,
        numbers: vec![String::from("7678282691")],
        salary: 12000,
        team: Team::ENGINEERING,
        tax_number: String::from("tax no"),
    };
    let ser: String = serde_json::to_string(&madhav).unwrap();
    println!("Serialized Data: {}", ser);
    // let deser: Person = serde_json::from_str(&ser).unwrap();
    let deser: Employee= serde_json::from_str(&ser).unwrap();
    
    println!("Deserialized Data: {:?}", deser);
}
