pub fn bzp_heap_adjust_down(heap: &mut [i32], weight: &[i32], n_heap: usize) {
    let mut pos = 1;
    let mut chpos = pos << 1;
    let tmpid = heap[pos];
    let tmpv = weight[tmpid as usize];
    
    while chpos <= n_heap {
        if (chpos | 1) <= n_heap && weight[heap[chpos] as usize] > weight[heap[chpos | 1] as usize] {
            chpos |= 1;
        }
        if tmpv < weight[heap[chpos] as usize] {
            break;
        }
        heap[pos] = heap[chpos];
        pos = chpos;
        chpos = pos << 1;
    }
    heap[pos] = tmpid;
}
