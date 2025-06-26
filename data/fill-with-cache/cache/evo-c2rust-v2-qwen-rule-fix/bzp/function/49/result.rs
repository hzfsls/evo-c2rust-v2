pub fn BzpShellSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut increments: Array<i32, 2> = arr![BZP_SHELL_SORT_INCREMENT1!(), BZP_SHELL_SORT_INCREMENT0!()];
    let mut i: i32;
    let mut j: i32;
    if (l >= r).as_bool() {
        return;
    }
    c_for!(let mut id: i32 = 0; id < BZP_SHELL_SORT_INCREMENT_NUMS!().cast(); id.suffix_plus_plus(); {
        let mut H: i32 = increments[id].cast();
        if (r - l + 1 <= H).as_bool() {
            continue;
        }
        i = l + H;
        while (i <= r).as_bool() {
            let mut tmpIdx: i32 = sortBlock[i].cast();
            let mut tmpVal: i32 = idx[tmpIdx].cast();
            j = i - H;
            while (j >= l).as_bool() && (idx[sortBlock[j]] > tmpVal).as_bool() {
                sortBlock[j + H] = sortBlock[j].cast();
                j -= H;
            }
            sortBlock[j + H] = tmpIdx.cast();
            i += 1;
        }
    });
}