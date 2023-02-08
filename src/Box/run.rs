use super::MyBox;

pub fn run(){
    let mybox = MyBox::MyBox::new(String::from("MyBox"));
    // Deref returns a refrence and not a value 
    // Because we do not want to pass the ownership of the 
    // data to the place we are derefrrencing it
    // We dont want a "MOVE"
    println!("{:?}", *mybox);
    sayWorrd(&mybox);
    // &MyBox<String>.deref() -> &String.deref() -> &str
    let mybox2 = MyBox::MyBox::new(1);
    let mybox3 = MyBox::MyBox::new(2);
    // drop(mybox);
    // Drop order is 2, 1, MyBox
    // If we want to drop one value before another
    // we can call the drop method
    drop(mybox);

}

fn sayWorrd(word: &str){
    // deref coercion example
    // Deref corecion does not work from immutable to mutable
    println!("{:?}", word);
}