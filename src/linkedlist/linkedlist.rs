// This is not even a linkedlist
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<T>
    where T: std::clone::Clone {
    pub value: T,
    next: Option<Box<Node<T>>>
}
impl<T: std::cmp::PartialEq + std::clone::Clone> Node<T> {
   pub fn new(value: T) -> Self {
        Node {value, next: None}
    }
    pub fn push(&mut self, value: T) {
        match self.next {
            Some(ref mut next_node) =>  {
                next_node.push(value);
            },
            None => {
                self.next = Some(Box::new(Node::new(value)))
            }
        }
    }
    pub fn length(&mut self) -> usize {
        let mut len_ctr = 1;
        let mut node_cpy = self.clone();        
        loop {
           if node_cpy.next == None {
                return len_ctr;
            }
            if node_cpy.next != None {
                node_cpy = *node_cpy.next.unwrap();
                len_ctr = len_ctr + 1;
            }
        }
    }
}
