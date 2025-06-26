pub fn binary_heap_pop(mut heap: Ptr<BinaryHeap>) -> BinaryHeapValue {
    let mut result: BinaryHeapValue = Default::default();
    let mut new_value: BinaryHeapValue = Default::default();
    let mut index: u32 = Default::default();
    let mut next_index: u32 = Default::default();
    let mut child1: u32 = Default::default();
    let mut child2: u32 = Default::default();

    if (heap.num_values == 0).as_bool() {
        return BINARY_HEAP_NULL!();
    }

    result = heap.values[0].cast();

    new_value = heap.values[heap.num_values - 1].cast();
    heap.num_values -= 1;

    index = 0;

    loop {
        child1 = index * 2 + 1;
        child2 = index * 2 + 2;

        if (child1 < heap.num_values).as_bool() && (binary_heap_cmp(heap.cast(), new_value.cast(), heap.values[child1].cast()) > 0) {
            if (child2 < heap.num_values).as_bool() && (binary_heap_cmp(heap.cast(), heap.values[child1].cast(), heap.values[child2].cast()) > 0).as_bool() {
                next_index = child2;
            } else {
                next_index = child1;
            }
        } else if (child2 < heap.num_values).as_bool() && (binary_heap_cmp(heap.cast(), new_value.cast(), heap.values[child2].cast()) > 0) {
            next_index = child2;
        } else {
            heap.values[index] = new_value.cast();
            break;
        }

        heap.values[index] = heap.values[next_index].cast();

        index = next_index;
    }

    return result.cast();
}
