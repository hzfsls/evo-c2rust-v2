pub fn BzpBinaryLiftingSort(mut bwt: Ptr<BzpBwtInfo>) {
    let mut ftab: Array<i32, { BZP_ASCII_SIZE!() }> = arr![0; BZP_ASCII_SIZE!()];
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock.cast(); i.suffix_plus_plus(); {
        ftab[bwt.block[i]] += 1;
    });
    c_for!(let mut i: i32 = 1; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock.cast(); i.suffix_plus_plus(); {
        let mut ch: i32 = bwt.block[i].cast();
        ftab[ch] -= 1;
        bwt.sortBlock[ftab[ch]] = i.cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        bwt.isStartPos[ftab[i]] = 1;
    });
    let mut M: i32 = 1;
    let mut sortflag: bool = true;

    while (M < bwt.nBlock.cast()).as_bool() && sortflag.as_bool() {
        let mut st: i32 = 0;
        sortflag = false;

        c_for!(let mut i: i32 = 0; i < bwt.nBlock.cast(); i.suffix_plus_plus(); {
            if bwt.isStartPos[i].as_bool() {
                st = i;
            }
            let mut pos: i32 = bwt.sortBlock[i] - M;
            if (pos < 0).as_bool() {
                pos += bwt.nBlock;
            }
            bwt.idx[pos] = st;
        });
        let mut l: i32 = 0;
        let mut r: i32 = 1;
        while (l < bwt.nBlock.cast()).as_bool() {
            while (r < bwt.nBlock.cast()).as_bool() && (bwt.isStartPos[r] != 1).as_bool() {
                r += 1;
            }
            r -= 1;
            if (l < r).as_bool() {
                sortflag = true;
                BzpQuickSort(bwt.sortBlock.cast(), bwt.idx.cast(), l.cast(), r.cast());
                BzpUpdateflag(bwt.cast(), l.cast(), r.cast());
            }
            l = r + 1;
            r = l + 1;
        }
        M <<= 1;
    }
}