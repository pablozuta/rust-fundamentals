// Import the BinaryHeap struct from the std::collections module
use std::collections::BinaryHeap;

fn main() {
    // Create a new BinaryHeap instance called 'heap'
    let mut heap = BinaryHeap::new();

    heap.push(3); // Push the value 3 onto the heap
    heap.push(2); // Push the value 2 onto the heap
    heap.push(1); // Push the value 1 onto the heap

    // Pop and print all elements on the heap in descending order
    while let Some(element) = heap.pop() {
        println!("{}", element); // output 3, 2, 1
    }
}