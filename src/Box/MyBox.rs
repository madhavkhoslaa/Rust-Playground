use std::ops::Deref;
use std::ops::Drop;

pub struct MyBox<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T: std::fmt::Debug> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: std::fmt::Debug> Drop for MyBox<T> {
    // This frees memory when a value goes out of scope
    // Drops data in reverse order in the scope
    // Destructor method
    fn drop(&mut self) {
        println!("Dropped data {:?}", self.0)
    }
}
