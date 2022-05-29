mod binary_heap;
mod binomial_heap;

use std::time::Instant;

use crate::binary_heap::BinaryHeap;
use crate::binomial_heap::BinomialHeap;

const HEAP_SIZES: [usize; 9] = [
    10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000
];

fn main() {
    let mut binary_heap_insert_times = vec![];
    let mut binomial_heap_insert_times = vec![];

    for i in &HEAP_SIZES {
        println!("Heap size: {}", *i); 
        let mut total_time = 0;
        for _ in 1..10 {
            let mut binomial_heap1 = BinomialHeap::new();
            for k in 0..*i {
                binomial_heap1.push(k,  k as f64);
            }
            
            let mut binomial_heap2 = BinomialHeap::new();
            for k in *i..(i*2) {
                binomial_heap2.push(k,  k as f64);
            }

            let start = Instant::now();
            binomial_heap1.meld(binomial_heap2);
            total_time += start.elapsed().as_nanos();
            
            if binomial_heap1.is_empty() {
                println!("Hey");
            }
        }
        binomial_heap_insert_times.push(total_time);

        let mut total_time = 0;
        for _ in 1..10 {
            let mut binary_heap1 = BinaryHeap::new();
            for k in 0..*i {
                binary_heap1.push(k, k as f64);
            }

            let mut binary_heap2 = BinaryHeap::new();
            for k in *i..(i*2) {
                binary_heap2.push(k, k as f64);
            }
            
            let start = Instant::now();
            binary_heap1.meld(binary_heap2);
            total_time += start.elapsed().as_nanos();
            
            if binary_heap1.is_empty() {
                println!("Hey");
            }
        }
        binary_heap_insert_times.push(total_time);
    }

    println!("n binary binomial");

    for i in 0..HEAP_SIZES.len() {
        println!("{} {} {}", HEAP_SIZES[i], binary_heap_insert_times[i], binomial_heap_insert_times[i]);
    }

}
