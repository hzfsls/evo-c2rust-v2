pub fn BzpShellSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let increments: [i32; 2] = [BZP_SHELL_SORT_INCREMENT1!(), BZP_SHELL_SORT_INCREMENT0!()];
    let mut i: i32;
    let mut j: i32;
    if l >= r {
        return;
    }
    c_for!(let mut id = 0; id < BZP_SHELL_SORT_INCREMENT_NUMS!(); id += 1; {
        let mut H = increments[id as usize];
        if r - l + 1 <= H {
            continue;
        }
        c_for!(i = l + H; i <= r; i += 1; {
            let mut tmpIdx = sortBlock[i];
            let mut tmpVal = idx[tmpIdx];
            c_for!(j = i - H; j >= l && idx[sortBlock[j]] > tmpVal; j -= H; {
                sortBlock[j + H] = sortBlock[j];
            });
            sortBlock[j + H] = tmpIdx;
        });
    });
}