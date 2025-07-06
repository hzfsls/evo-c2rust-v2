pub fn BzpMtfMain(mut mtf: Ptr<BzpMtfInfo>) {
    let mut list: Array<u8, { BZP_MAX_ALPHA_SIZE!() }> = Default::default();
    let mut EOB: i32 = Default::default();
    let mut num: i32 = 0;
    BzpMapInputChar(mtf, list.cast(), BZP_MAX_ALPHA_SIZE!());
    EOB = (mtf.nUse + 1);
    c_for!(let mut i: i32 = 0; i <= EOB; i.suffix_plus_plus(); {
        mtf.mtfFreq[i] = 0;
    });
    c_for!(let mut i: i32 = 0; i < mtf.nBlock; i.suffix_plus_plus(); {
        let mut pos: i32 = (mtf.map[i] - 1);
        if (pos < 0) {
            pos += mtf.nBlock;
        }
        let mut ch: u8 = mtf.block[pos];
        if (ch == list[0]) {
            num += 1;
        } else {
            if (num > 0) {
                BzpNumEncode(mtf, num);
                num = 0;
            }
            let mut pos_: i32 = 1;
            while (ch != list[pos_]) && (pos_ < mtf.nUse) {
                pos_ += 1;
            }
            c_for!(let mut j: i32 = pos_; j > 0; j.suffix_minus_minus(); {
                list[j] = list[j - 1];
            });
            list[0] = ch;
            let tmp0 = mtf.nMtf;
            mtf.mtfV[tmp0] = (pos_ + 1);
            mtf.mtfFreq[pos_ + 1] += 1;
            mtf.nMtf += 1;
        }
    });
    if (num > 0) {
        BzpNumEncode(mtf, num);
    }
    let tmp0 = mtf.nMtf;
    mtf.mtfV[tmp0] = EOB;
    mtf.mtfFreq[EOB] += 1;
    mtf.nMtf += 1;
}
