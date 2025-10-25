use heap_rust::heap::Heap;
use heap_rust::heap::NewTrait;

use ordered_float::OrderedFloat;

#[test]
fn heap_test() {
    // This will now work
    let mut int_heap: Heap<i32> = NewTrait::<i32>::new();
    int_heap.push(10);
    int_heap.push(5);
    println!("{:?}", int_heap);

    // This will also work
    let mut float_heap: Heap<OrderedFloat<f64>> = Heap::<OrderedFloat<f64>>::new();
    float_heap.push(OrderedFloat(1.1));
    float_heap.push(OrderedFloat(9.9));
    println!("{:?}", float_heap);
}
