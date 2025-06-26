pub fn BzpUpdateflag(mut bwt: Ptr<BzpBwtInfo>, mut l: i32, mut r: i32) {
    let mut tmpst: i32 = -1;
    c_for!(let mut i: i32 = l; i <= r; i.suffix_plus_plus(); {
        let mut tmpnow: i32 = bwt.idx[bwt.sortBlock[i]];
        if (tmpst != tmpnow).as_bool() {
            bwt.isStartPos[i] = 1;
            tmpst = tmpnow;
        }
    });
}
