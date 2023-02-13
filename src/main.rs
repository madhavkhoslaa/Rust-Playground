use rust_playground::interiormutability;
use rust_playground::linkedlist;
use rust_playground::Box;
use rust_playground::SerdePlayground;
use rust_playground::RC;

fn main() {
    println!("Serde Playground");
    SerdePlayground::run::run();
    println!("___________________________________________");
    println!("Linkedlist Playground");
    linkedlist::run::run();
    println!("___________________________________________");
    println!("Run Box Playground");
    Box::Box::RunBox();
    println!("___________________________________________");
    println!("Run Box Playground");
    Box::run::run();
    println!("___________________________________________");
    println!("Run Box Playground");
    RC::rc::run();
    println!("___________________________________________");
    println!("Run RefCell Playground");
    interiormutability::run::run()
}
