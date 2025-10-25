use num::Num;
use std::fmt::Debug;

// 1. Define the struct to be generic over a type `T`.
//    A heap is a collection, so it usually holds a `Vec<T>`.
#[derive(Debug)]
pub struct Heap<T> {
    data: Vec<T>,
}

pub trait NewTrait<T>
where
    T: Num + Ord + Debug + Clone,
{
    // Heap impl
    fn new() -> Self;
    fn sift_up(&mut self);
    fn sift_down(&mut self);
    fn peek(self, i: usize) -> T;
    // Your methods (like `push`, `pop`) would go here.
    // They rely on `T` being `Ord` to sift elements up or down.
    fn push(&mut self, value: T);
    fn pop(&mut self, i: usize) -> T;
    fn len(&mut self) -> usize;
    fn is_empty(self) -> bool;
}

// 2. Apply the constraints on the `impl` block where you define
//    the heap's behavior.
//
//    - T: Num    -> Ensures T is a "number" from the `num` crate.
//    - T: Ord    -> Ensures T can be ordered (critical for a heap).
//    - T: Debug  -> Allows the struct to be printed (from your `derive`).
impl<T> NewTrait<T> for Heap<T>
where
    T: Num + Ord + Debug + Clone,
{
    // Heap impl
    fn new() -> Self {
        Heap { data: Vec::new() }
    }
    fn sift_up(&mut self) {
        let mut i: usize = self.len() - 1;

        while i > 0 {
            let father_i: usize = (i - 1) >> 1;
            if self.data[i] < self.data[father_i] {
                self.data.swap(i, father_i);
                i = father_i;
            } else {
                break;
            }
        }
    }
    fn sift_down(&mut self) {
        let mut i: usize = 0;
        let mut left_i: usize;
        let mut right_i: usize;
        let mut smallest_i: usize = 0;

        let len = self.len();
        while i < len {
            left_i = (i << 1) + 1;
            right_i = left_i + 1;
            if left_i < len && self.data[i] > self.data[left_i] {
                smallest_i = left_i;
            }
            if right_i < len && self.data[i] > self.data[right_i] {
                smallest_i = right_i;
            }
            if smallest_i != i {
                self.data.swap(i, smallest_i);
                i = smallest_i
            } else {
                break;
            }
        }
    }
    fn peek(self, i: usize) -> T {
        self.data[i].clone()
    }
    // Your methods (like `push`, `pop`) would go here.
    // They rely on `T` being `Ord` to sift elements up or down.
    fn push(&mut self, value: T) {
        self.data.push(value);
        self.sift_up();
    }
    fn pop(&mut self, i: usize) -> T {
        let a = self.data.swap_remove(i);
        self.sift_down();
        a
    }
    fn len(&mut self) -> usize {
        self.data.len()
    }
    fn is_empty(self) -> bool {
        self.data.len() == 0
    }
}

impl<T: Num + Ord + Debug + Clone> Default for Heap<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Num + Ord + Debug + Clone> Iterator for Heap<T> {
    // `type Item` belongs *here*
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // The iterator just calls `pop` until the heap is empty
        Some(self.pop(0))
    }
}
#[cfg(test)]
mod tests {
    // Import everything from the parent module (this file)
    use super::*;
    // You'd also import test-only crates here
    use ordered_float::OrderedFloat;

    #[test]
    fn test_heap_push() {
        let mut int_heap = Heap::<i32>::new();
        int_heap.push(10);
        int_heap.push(5);
        // Tests should use assertions, not just print
        assert_eq!(int_heap.len(), 2);
    }
    #[test]
    fn test_heap_peek() {
        let mut int_heap = Heap::<i32>::new();
        int_heap.push(10);
        int_heap.push(5);
        // Tests should use assertions, not just print
        assert_eq!(int_heap.peek(0), 5);
    }
    #[test]
    fn test_float_heap() {
        let mut float_heap = Heap::<OrderedFloat<f64>>::new();
        float_heap.push(OrderedFloat(1.1));
        float_heap.push(OrderedFloat(9.9));
        assert_eq!(float_heap.len(), 2);
    }
}
