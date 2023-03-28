use fibheap::*;
use rand::prelude::*;
use std::iter::*;

fn main() {
    let mut rng = thread_rng();
    let mut heap = repeat_with(|| rng.gen_range(0..1000u32))
        .take(1_000_000)
        .collect::<v2::FibonacciHeap<_>>();

    loop {
        if heap.pop().is_none() {
            break;
        }
    }
}
