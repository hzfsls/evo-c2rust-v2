pub fn BzpWriteValidASCII(mut outData: Ptr<BzpOutComdata>, mut bwt: Ptr<BzpBwtInfo>) {
    let mut validGid: Array<i32, { BZP_ASCII_SIZE!() }> = Default::default();
    let mut cnt: i32 = 0;
    let mut use16: Array<bool, { BZP_ASCII_SIZE!() }> = Default::default();
    c_memset_s!(use16, c_sizeofval!(use16), 0, c_sizeofval!(use16));
    c_for!(let mut i = 0; i < BZP_ASCII_SIZE!(); i += 1; {
        let mut gid: i32 = i / BZP_CHARS_PER_GROUP_ASCII!();
        use16[gid] |= bwt.inUse[i];
    });
    c_for!(let mut i = 0; i < BZP_GROUPS_ASCII!(); i += 1; {
        BzpWriteToArray(use16[i] as i32, BZP_BIT!(), outData);
        if use16[i] {
            validGid[cnt] = i;
            cnt += 1;
        }
    });
    c_for!(let mut i = 0; i < cnt; i += 1; {
        c_for!(let mut j = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j += 1; {
            let mut valid: i32 = validGid[i] * BZP_CHARS_PER_GROUP_ASCII!() + j;
            BzpWriteToArray(bwt.inUse[valid] as i32, BZP_BIT!(), outData);
        });
    });
}