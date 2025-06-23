pub fn BzpHeapAdjustUp(mut heap: Ptr<i32>, mut weight: Ptr<i32>, mut pos: i32) {
    let mut tmpw: i32 = weight[heap[pos]].cast();
    let mut tmpv: i32 = heap[pos].cast();
    while (pos > 1).as_bool() {
        if (tmpw < weight[heap[pos >> 1]]).as_bool() {
            heap[pos] = heap[pos >> 1].cast();
            pos >>= 1;
        } else {
            break;
        }
    }
    heap[pos] = tmpv.cast();
}
