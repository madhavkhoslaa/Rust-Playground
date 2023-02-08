use crate::linkedlist::linkedlist::Node;

pub fn run() {
    let mut head = Node::new(1);
    head.push(2);
    head.push(3);
    head.push(4);
    head.push(5);
    head.push(6);
    head.push(7);
    println!("List {:?}", head);
    println!("List length {:?}", head.length());
    head.pop();
    println!("List after pop {:?}", head);
    println!("List length after pop {:?}", head.length());
}
