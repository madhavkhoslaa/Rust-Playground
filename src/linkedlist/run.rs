use crate::linkedlist::linkedlist::Node;

pub(crate) fn run() {
    let mut head = Node::new("Madhav");
    head.push("Diya");
    head.push("Based");
    println!("Print Linkedlist:  {:?}", head);
    println!("Length of linkedlist:  {:?}", head.length());
    println!("Based");
}