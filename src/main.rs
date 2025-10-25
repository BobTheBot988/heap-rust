// 1. Tell main.rs about your heap.rs file
mod heap;

// 2. Bring Heap into scope (optional, but convenient)
use heap::Heap;
use num::Num; // Needed for the trait bounds on ::new()

fn main() {
    // 3. Now, actually *use* the heap!
    // This code is no longer "dead".
    let mut int_heap = Heap::<i32>::new();
    int_heap.push(10);
    int_heap.push(5);

    println!("My heap from main.rs: {:?}", int_heap);
}
