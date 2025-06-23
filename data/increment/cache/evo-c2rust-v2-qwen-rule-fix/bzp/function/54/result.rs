pub fn BzpBwtDecode(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    let mut ftab: Array<i32, 257> = arr![0; 257];
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        ftab[bwt.block[i].cast::<u8>().cast::<i32>() + 1] += 1;
    });
    c_for!(let mut i: i32 = 1; i <= BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        let mut ch: u8 = bwt.block[i];
        bwt.sorted[ftab[ch.cast::<i32>()]] = i;
        let tmp0 = ch.cast::<i32>();
        ftab[tmp0] += 1;
    });
    let mut cnt: i32 = 0;
    let mut pos: i32 = bwt.oriPtr;
    while (cnt < bwt.nBlock) {
        pos = bwt.sorted[pos];
        let mut ch: u8 = bwt.block[pos];
        bwt.deCode[cnt] = ch;
        cnt += 1;
    }
}