pub fn BzpBwtDecode(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    let mut ftab: Array<i32, 257> = arr![0; 257];
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        ftab[bwt.block[i] + 1] += 1;
    });
    c_for!(let mut i: i32 = 1; i <= BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        let mut ch: u8 = bwt.block[i].cast();
        bwt.sorted[ftab[ch]] = i.cast();
        ftab[ch] += 1;
    });
    let mut cnt: i32 = 0;
    let mut pos: i32 = bwt.oriPtr.cast();
    while cnt < bwt.nBlock {
        pos = bwt.sorted[pos].cast();
        let mut ch: u8 = bwt.block[pos].cast();
        bwt.deCode[cnt] = ch.cast();
        cnt += 1;
    }
}
