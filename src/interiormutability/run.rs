use std::cell::RefCell;

use super::refcell::{Student, UpdateGrades};

pub fn run() {
    let x: Student = Student::new();
    x.update(vec![1, 2, 3, 4]);
    println!("{:?}", x);
}
