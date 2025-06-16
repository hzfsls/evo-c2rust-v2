pub fn BzpNumEncode(mut mtf: Ptr<BzpMtfInfo>, mut num: i32) {
    num <<= 1;
    c_do!({
        num >>= 1;
        num -= 1;
        if (num & 1).as_bool() {
            mtf.mtfV[mtf.nMtf] = BZP_MTF_ENCODE1!();
            mtf.nMtf += 1;
            mtf.mtfFreq[BZP_MTF_ENCODE1!()] += 1;
        } else {
            mtf.mtfV[mtf.nMtf] = BZP_MTF_ENCODE0!();
            mtf.nMtf += 1;
            mtf.mtfFreq[BZP_MTF_ENCODE0!()] += 1;
        }
    } while num >= BZP_MTF_ENCODE_BASE!());
}
