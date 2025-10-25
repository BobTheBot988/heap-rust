use "../src/heap.rs";
#[cfg(test)]
mod test {
    #[test]
    fn heap_test() {
        // This works because `i32` implements `Num + Ord + Debug`
        let mut int_heap = Heap::<i32>::new();
        int_heap.push(10);
        int_heap.push(5);
        println!("{:?}", int_heap);

        // This also works because `f64` implements `Num + Ord + Debug`
        // (Note: FLoat ordering can be tricky with NaN, but it does implement Ord)
        let mut float_heap = Heap::<OrderedFloat<f64>>::new();
        float_heap.push(OrderedFloat(1.1));
        float_heap.push(OrderedFloat(9.9));
        println!("{:?}", float_heap);
    }
}
