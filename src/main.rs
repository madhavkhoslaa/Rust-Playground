mod Box;
mod RC;
mod SerdePlayground;
mod linkedlist;

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
}
