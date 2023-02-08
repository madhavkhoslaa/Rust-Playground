// Box is a type that stores data in heap
// Hence it follows the borrowing rules
// It implements the drop and deref traits from the rust library
// Deref traits allows to be trated like refrences
// Drop traits runs when the instance is out of scope
// Many libraries implement their own smart pointers
// No overhead or  capabilities
//
// WHen to use?
// WHen you dont know the size of the data
// Transfer large data without copying
// When you care about a type that is returned from the function implementing certain traits
//

pub fn RunBox() {
    let a = Box::new(1);
    println!("value in Box a{:?}", a);
    let b = a;
    println!("Value moved to variable b {:?}", b);
    // Now b owns the value in a;
    // println!("{:?}", a) will result in an error because the value is moved to b
    // How does the data from a is printed witout derefrencing the data using * like in c++ ??
}
