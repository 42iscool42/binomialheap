mod binary_heap;

use std::fmt::Display;
use std::hash::Hash;

use binary_heap::BinaryHeap;

fn main() {
    println!("Hello, world!");

    let mut heap = BinaryHeap::<str>::new();

    let neg_one = "-1";

    heap.push(&"5", 5.0);
    heap.push(&"4", 4.0);
    heap.push(&"3", 3.0);
    heap.push(&"2", 2.0);
    heap.push(&"1", 1.0);
    heap.push(&neg_one, 50.0);
    peek_and_print(&heap);

    heap.update_weight(&"-1", -10.0);
    peek_and_print(&heap);

    heap.update_weight(&neg_one, 3.5);
    peek_and_print(&heap);

    heap.update_weight(&neg_one, -1.0);


    while !heap.is_empty() {
        pop_and_print(&mut heap);
    }
}

fn peek_and_print<V: Eq + Hash + Display + ?Sized>(heap: &BinaryHeap<V>) {
    println!("{}", heap.peek().expect("Heap empty").value);
}

fn pop_and_print<V: Eq + Hash + Display + ?Sized>(heap: &mut BinaryHeap<V>) {
    println!("{}", heap.pop().expect("Heap empty").value);
}