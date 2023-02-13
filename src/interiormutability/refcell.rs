// it is a design pattern that allows data in a immutable refrence to be mutated
// this uses an unsafe code in a wrapper
// unsafe code is not checked at compile time for memory saftey
// it is enforced at runtime
// Single ownership over the data like box
// Except refcell inforces borrowing rules at runtime
// it panics in runtime
// Halting problem <- read
// only in single threaded
// mutating a vlaue inside a immutable structure is known as interior mutability
//
//
// Example
// let a = 10;
// let b = mut &a;
// THis is not possible since a is not mutable and you cannot create a mutable refrence to a
// immutable variable
//
// Example 2
// let myt a = 10;
// let d = &a;
// let *d =20;
//
//This does not work because we have a mutable refrence
//
// We cannot have a immutable refrence change mutable data at borrowtime in rust
//s
// It's like you have a immutable refrence to a ds and it holda mutable values and you have
// functions that can mutate the interior value. a refcell is similar in this way
//
//

use std::cell::RefCell;

pub trait UpdateGrades {
    fn update(&self, grades: Vec<usize>);
}

#[derive(Debug)]
pub struct Student {
    grades: RefCell<Vec<usize>>,
}

impl Student {
    pub fn new() -> Student {
        Student {
            grades: RefCell::new(vec![]),
        }
    }
}

impl UpdateGrades for Student {
    fn update(&self, grades: Vec<usize>) {
        for grade in grades.iter() {
            self.grades.borrow_mut().push(*grade);
        }
    }
    // // This will panic!
    // fn update(&self, grades: Vec<usize>) {

    //  let mut a = self.grades.borrow_mut();
    //  let mut b = self.grades.borrow_mut();
    //  a.push(1);
    //  b.push(2);
    // }
}
