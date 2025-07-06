pub fn BzpBwtDecode(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    let mut ftab: Array<i32, 257> = Default::default();
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab));
    c_for!(let mut i = 0; i < bwt.nBlock; i += 1; {
        ftab[bwt.block[i] as usize + 1] += 1;
    });
    c_for!(let mut i = 1; i <= BZP_ASCII_SIZE!(); i += 1; {
        ftab[i as usize] += ftab[i as usize - 1];
    });
    c_for!(let mut i = 0; i < bwt.nBlock; i += 1; {
        let ch: u8 = bwt.block[i];
        bwt.sorted[ftab[ch as usize] as usize] = i;
        ftab[ch as usize] += 1;
    });
    let mut cnt: i32 = 0;
    let mut pos: i32 = bwt.oriPtr;
    while cnt < bwt.nBlock {
        pos = bwt.sorted[pos as usize];
        let ch: u8 = bwt.block[pos as usize];
        bwt.deCode[cnt as usize] = ch;
        cnt += 1;
    }
}