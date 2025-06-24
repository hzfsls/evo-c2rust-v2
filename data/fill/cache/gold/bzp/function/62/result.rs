pub fn BzpNumEncode(mut mtf: Ptr<BzpMtfInfo>, mut num: i32) {
    num <<= 1;
    loop {
        num >>= 1;
        num -= 1;
        if num & 1 != 0 {
            index!(mtf.mtfV, mtf.nMtf, BZP_MTF_ENCODE1!());
            mtf.mtfFreq[BZP_MTF_ENCODE1!()] += 1;
        } else {
            index!(mtf.mtfV, mtf.nMtf, BZP_MTF_ENCODE0!());
            mtf.mtfFreq[BZP_MTF_ENCODE0!()] += 1;
        }
        mtf.nMtf += 1;
        if num < BZP_MTF_ENCODE_BASE!() {
            break;
        }
    }
}