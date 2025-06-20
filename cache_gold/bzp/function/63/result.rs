pub fn BzpMtfMain(mut mtf: Ptr<BzpMtfInfo>) {
    let mut list: Array<u8, { BZP_MAX_ALPHA_SIZE!() }> = Default::default();
    let mut EOB: i32;
    let mut num: i32 = 0;
    BzpMapInputChar(mtf, list.cast(), BZP_MAX_ALPHA_SIZE!());
    EOB = mtf.nUse + 1;
    c_for!(let mut i = 0; i <= EOB; i += 1; {
        mtf.mtfFreq[i] = 0;
    });
    c_for!(let mut i = 0; i < mtf.nBlock; i += 1; {
        let mut pos = mtf.map[i] - 1;
        if pos < 0 {
            pos += mtf.nBlock;
        }
        let mut ch = mtf.block[pos];
        if ch == list[0] {
            num += 1;
        } else {
            if num > 0 {
                BzpNumEncode(mtf, num);
                num = 0;
            }
            let mut pos_ = 1;
            while ch != list[pos_] && pos_ < mtf.nUse {
                pos_ += 1;
            }
            c_for!(let mut j = pos_; j > 0; j -= 1; {
                list[j] = list[j - 1];
            });
            list[0] = ch;
            // mtf.mtfV[mtf.nMtf] = pos_ + 1;
            index!(mtf.mtfV, mtf.nMtf, pos_ + 1);
            mtf.mtfFreq[pos_ + 1] += 1;
            mtf.nMtf += 1;
        }
    });
    if num > 0 {
        BzpNumEncode(mtf, num);
    }
    index!(mtf.mtfV, mtf.nMtf, EOB);
    mtf.mtfFreq[EOB] += 1;
    mtf.nMtf += 1;
}