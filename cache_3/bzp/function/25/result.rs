pub fn BzpQuickSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut stack: BzpQSortInfo = Default::default();
    stack.cnt = 0;
    stack.stackL[stack.cnt] = l.cast();
    stack.stackR[stack.cnt] = r.cast();
    stack.cnt += 1;
    while (stack.cnt > 0).as_bool() {
        stack.cnt -= 1;
        let mut tl: i32 = stack.stackL[stack.cnt].cast();
        let mut tr: i32 = stack.stackR[stack.cnt].cast();
        if (tl >= tr).as_bool() {
            continue;
        }
        if (tr - tl < BZP_THRESHOLD_SHELL_SORT!()).as_bool() {
            BzpShellSort(sortBlock.cast(), idx.cast(), tl.cast(), tr.cast());
            continue;
        }
        stack.tl = tl.cast();
        stack.tr = tr.cast();
        BzpQSortSingle(sortBlock.cast(), idx.cast(), c_ref!(stack).cast());
    }
}
