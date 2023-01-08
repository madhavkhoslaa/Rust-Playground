use crate::SerdePlayground::{self, person::Person};

pub fn run(){
    let madhav = SerdePlayground::person::Person {
        name: String::from("Madhav Khosla"),
        age: 23,
        occupation: String::from("Engineer"),
        married: SerdePlayground::person::Married::NO
    };
    let ser: String = serde_json::to_string(&madhav).unwrap();
    println!("Serialized Data: {}", ser);
    let deser: Person = serde_json::from_str(&ser).unwrap();
    println!("Deserialized Data: {:?}", deser);
}