pub fn bzp_update_flag(bwt: &mut BzpBwtInfo, l: i32, r: i32) {
    let mut tmpst = -1;
    for i in l..=r {
        let tmpnow = bwt.idx[bwt.sort_block[i as usize] as usize];
        if tmpst != tmpnow {
            bwt.is_start_pos[i as usize] = 1;
            tmpst = tmpnow;
        }
    }
}
