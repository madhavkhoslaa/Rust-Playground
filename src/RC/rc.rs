// In most of the cases the values are owned by a single owner
// In cases like graphs and double linkedlists where the node(data struct) is owned by multiple values
// We need to use an RC
// In this case the node should not be freed until all of it's owneship is removed
// RC hold the count of refrences to a structres and drops it when there are no refrences
// Only signle threaded programs

use std::rc::Rc;

struct Node {
    value: i32,
    Next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32, Next: Option<Box<Node>>) -> Node {
        Node { value, Next }
    }
}

#[derive(Debug)]
struct NodeRC {
    value: i32,
    Next: Option<Rc<NodeRC>>,
}

impl NodeRC {
    pub fn new(value: i32, Next: Option<Rc<NodeRC>>) -> NodeRC {
        NodeRC { value, Next }
    }
}
impl Drop for NodeRC {
    fn drop(&mut self) {
        println!("Node is dropped {:?}", self);
    }
}
pub fn run() {
    // let a = Node::new(1, None);
    // let c = Node::new(1, Some(Box::new(a)));
    // let b = Node::new(1,Some(Box::new(a)));
    // This does not work because of borrow checker rules
    //  we want both c and b to point to a
    let a = Rc::new(NodeRC::new(1, None));
    // a refrence count is 1
    let b = NodeRC::new(2, Some(Rc::clone(&a)));
    // a refrence count is 2
    let c = NodeRC::new(3, Some(Rc::clone(&a)));
    // a refrence count is 3
    println!("a is {:?}", a);
    println!("b is {:?}", b);
    println!("c is {:?}", c);
    println!("a refrence count is {}", Rc::strong_count(&a));
    // Rc only sends immmutable refrences because of multiple owners we can result in a race condition and break borrowing rules

    // Rc::clone does not create new copies of the data like other impls of clone
    // Shallow copy
}
