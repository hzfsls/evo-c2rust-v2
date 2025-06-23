pub fn BzpGetHuffmanGroups(mut nBlock: i32) -> i32 {
    let mut nGroups: i32 = 1;
    if nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT0!() {
        nGroups = BZP_NGROUPS_NUM_0!();
    } else if nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT1!() {
        nGroups = BZP_NGROUPS_NUM_1!();
    } else if nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT2!() {
        nGroups = BZP_NGROUPS_NUM_2!();
    } else if nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT3!() {
        nGroups = BZP_NGROUPS_NUM_3!();
    } else {
        nGroups = BZP_NGROUPS_NUM_4!();
    }
    return nGroups.cast();
}
