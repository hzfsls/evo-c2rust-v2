pub fn BzpShellSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut increments: Array<i32, 2> = arr![BZP_SHELL_SORT_INCREMENT1!(), BZP_SHELL_SORT_INCREMENT0!()];
    let mut i: i32 = Default::default();
    let mut j: i32 = Default::default();
    if l >= r {
        return;
    }
    c_for!(let mut id: i32 = 0; id < BZP_SHELL_SORT_INCREMENT_NUMS!(); id.suffix_plus_plus(); {
        let mut H: i32 = increments[id];
        if r - l + 1 <= H {
            continue;
        }
        c_for!(i = l + H; i <= r; i.suffix_plus_plus(); {
            let mut tmpIdx: i32 = sortBlock[i];
            let mut tmpVal: i32 = idx[tmpIdx];
            c_for!(j = i - H; j >= l && idx[sortBlock[j]] > tmpVal; j -= H; {
                sortBlock[j + H] = sortBlock[j];
            });
            sortBlock[j + H] = tmpIdx;
        });
    });
}
