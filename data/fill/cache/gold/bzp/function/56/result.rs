pub fn BzpBinaryLiftingSort(mut bwt: Ptr<BzpBwtInfo>) {
    let mut ftab: Array<i32, { BZP_ASCII_SIZE!() } > = Default::default();
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab));
    c_for!(let mut i = 0; i < bwt.nBlock; i += 1; {
        ftab[bwt.block[i]] += 1;
    });
    c_for!(let mut i = 1; i < BZP_ASCII_SIZE!(); i += 1; {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i = 0; i < bwt.nBlock; i += 1; {
        let ch = bwt.block[i];
        ftab[ch] -= 1;
        bwt.sortBlock[ftab[ch]] = i;
    });
    c_for!(let mut i = 0; i < BZP_ASCII_SIZE!(); i += 1; {
        bwt.isStartPos[ftab[i]] = 1;
    });
    let mut M = 1;
    let mut sortflag = true;
    while M < bwt.nBlock && sortflag {
        let mut st = 0;
        sortflag = false;
        c_for!(let mut i = 0; i < bwt.nBlock; i += 1; {
            if bwt.isStartPos[i] == 1 {
                st = i;
            }
            let mut pos = bwt.sortBlock[i] - M;
            if pos < 0 {
                pos += bwt.nBlock;
            }
            bwt.idx[pos] = st;
        });
        let mut l = 0;
        let mut r = 1;
        while l < bwt.nBlock {
            while r < bwt.nBlock && bwt.isStartPos[r] != 1 {
                r += 1;
            }
            r -= 1;
            if l < r {
                sortflag = true;
                BzpQuickSort(bwt.sortBlock, bwt.idx, l, r);
                BzpUpdateflag(bwt, l, r);
            }
            l = r + 1;
            r = l + 1;
        }
        M <<= 1;
    }
}