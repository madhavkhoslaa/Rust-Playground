// Serde with attributes
// Using defaults
// deta format key names

// What are attributes ?
// customise how ser and deser implementation are produced by derive

// Types of attributes ?
// Container attributes => Applied to a struct
// Variant attributes => Applied to a varient of an enum
// Field attributes => apply to field in a struct or an enum

use serde::{Deserialize, Serialize};

/**
 * #[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // <-- this is a container attribute
struct S {
    #[serde(default)]  // <-- this is a field attribute
    f: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "e")]  // <-- this is also a container attribute
enum E {
    #[serde(rename = "a")]  // <-- this is a variant attribute
    A(String),
}
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub numbers: Vec<String>,
    pub salary: u16,
    pub team: Team,
    #[serde(skip)]
    pub tax_number: String, // this does not log tax number
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Team {
    OPERATIONS,
    ENGINEERING,
    PRODUCT,
    QA,
    SALES,
    MARKETING,
    HOUSEKEEPING,
}
