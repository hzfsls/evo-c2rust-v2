pub fn BzpQuickSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut stack: BzpQSortInfo = Default::default();
    stack.cnt = 0;
    stack.stackL[stack.cnt] = l;
    stack.stackR[stack.cnt] = r;
    stack.cnt += 1;
    while (stack.cnt > 0).as_bool() {
        stack.cnt -= 1;
        let mut tl: i32 = stack.stackL[stack.cnt];
        let mut tr: i32 = stack.stackR[stack.cnt];
        if (tl >= tr).as_bool() {
            continue;
        }
        if (tr - tl < BZP_THRESHOLD_SHELL_SORT!()).as_bool() {
            BzpShellSort(sortBlock.cast(), idx.cast(), tl.cast(), tr.cast());
            continue;
        }
        stack.tl = tl;
        stack.tr = tr;
        BzpQSortSingle(sortBlock.cast(), idx.cast(), c_ref!(stack).cast());
    }
}