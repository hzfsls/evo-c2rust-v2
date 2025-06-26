pub fn BzpQuickSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut stack: BzpQSortInfo = Default::default();
    stack.cnt = 0;
    stack.stackL[stack.cnt] = l;
    stack.stackR[stack.cnt] = r;
    stack.cnt += 1;
    while stack.cnt > 0 {
        stack.cnt -= 1;
        let mut tl = stack.stackL[stack.cnt];
        let mut tr = stack.stackR[stack.cnt];
        if tl >= tr {
            continue;
        }
        if tr - tl < BZP_THRESHOLD_SHELL_SORT!() {
            BzpShellSort(sortBlock, idx, tl, tr);
            continue;
        }
        stack.tl = tl;
        stack.tr = tr;
        BzpQSortSingle(sortBlock, idx, c_ref!(stack));
    }
}