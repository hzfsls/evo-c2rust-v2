pub fn bzp_heap_adjust_up(heap: &mut [i32], weight: &[i32], pos: usize) {
    let tmpw = weight[heap[pos] as usize];
    let tmpv = heap[pos];
    let mut current_pos = pos;
    while current_pos > 1 {
        let parent_pos = current_pos >> 1;
        if tmpw < weight[heap[parent_pos] as usize] {
            heap[current_pos] = heap[parent_pos];
            current_pos = parent_pos;
        } else {
            break;
        }
    }
    heap[current_pos] = tmpv;
}
