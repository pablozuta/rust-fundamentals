// This program creates a VecDeque, inserts some elements into it
// and then pops elements from the front of the deque until it is empty.  
use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);

    // prints all elements
    for element in &deque  {
        println!("{}", element);        
    }

    // pop element from the front of the deque
    while let Some(element) = deque.pop_front() {
        println!("popped element {}", element);
    }
}