pub fn BzpWriteValidASCII(mut outData: Ptr<BzpOutComdata>, mut bwt: Ptr<BzpBwtInfo>) {
    let mut validGid: Array<i32, { BZP_ASCII_SIZE!() }> = arr![0; BZP_ASCII_SIZE!()];
    let mut cnt: i32 = 0;
    let mut use16: Array<bool, { BZP_ASCII_SIZE!() }> = arr![false; BZP_ASCII_SIZE!()];
    c_memset_s!(use16, c_sizeofval!(use16), 0, c_sizeofval!(use16)).cast::<Void>();

    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        let mut gid: i32 = i / BZP_CHARS_PER_GROUP_ASCII!();
        let tmp0 = gid;
        use16[tmp0];
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!(); i.suffix_plus_plus(); {
        BzpWriteToArray(use16[i].cast::<i32>(), BZP_BIT!(), outData);
        if use16[i] {
            validGid[cnt.suffix_plus_plus()] = i;
        }
    });
    c_for!(let mut i: i32 = 0; i < cnt; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
            let mut valid: i32 = validGid[i] * BZP_CHARS_PER_GROUP_ASCII!() + j;
            BzpWriteToArray(bwt.inUse[valid].cast::<i32>(), BZP_BIT!(), outData);
        });
    });
}