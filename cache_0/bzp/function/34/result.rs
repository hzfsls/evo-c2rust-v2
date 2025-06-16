pub fn BzpMtfMain(mut mtf: Ptr<BzpMtfInfo>) {
    let mut list: Array<u8, { BZP_MAX_ALPHA_SIZE!() }> = Default::default();
    let mut EOB: i32 = Default::default();
    let mut num: i32 = 0;
    BzpMapInputChar(mtf.cast(), list.cast(), BZP_MAX_ALPHA_SIZE!().cast());
    EOB = (mtf.nUse + 1).cast();
    c_for!(let mut i: i32 = 0; i <= EOB; i.suffix_plus_plus(); {
        mtf.mtfFreq[i] = 0;
    });
    c_for!(let mut i: i32 = 0; i < mtf.nBlock; i.suffix_plus_plus(); {
        let mut pos: i32 = (mtf.map[i] - 1).cast();
        if (pos < 0).as_bool() {
            pos += mtf.nBlock;
        }
        let mut ch: u8 = mtf.block[pos].cast();
        if (ch == list[0]).as_bool() {
            num += 1;
        } else {
            if (num > 0).as_bool() {
                BzpNumEncode(mtf.cast(), num.cast());
                num = 0;
            }
            let mut pos_: i32 = 1;
            while (ch != list[pos_]).as_bool() && (pos_ < mtf.nUse).as_bool() {
                pos_ += 1;
            }
            c_for!(let mut j: i32 = pos_; j > 0; j.suffix_minus_minus(); {
                list[j] = list[j - 1].cast();
            });
            list[0] = ch.cast();
            mtf.mtfV[mtf.nMtf] = (pos_ + 1).cast();
            mtf.mtfFreq[pos_ + 1] += 1;
            mtf.nMtf += 1;
        }
    });
    if (num > 0).as_bool() {
        BzpNumEncode(mtf.cast(), num.cast());
    }
    mtf.mtfV[mtf.nMtf] = EOB.cast();
    mtf.mtfFreq[EOB] += 1;
    mtf.nMtf += 1;
}
