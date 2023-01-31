use rust_playground::SerdePlayground::person::Person;
mod SerdePlayground;
mod linkedlist;
fn main() {
    print!("Serde Playground");
    SerdePlayground::run::run();
    print!("Linkedlist Playground");
    linkedlist::run::run();
}
