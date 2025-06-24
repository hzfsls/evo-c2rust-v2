pub fn CmptlzMfGenHashTable(mut mf: Ptr<CmptMfCtx>) {
    let mut hashRootTable: Ptr<u32> = mf.hashRootTable.cast();
    let mut poly32: u32 = 0xEDB88320;
    let mut i: u32 = Default::default();
    let mut j: u32 = Default::default();
    c_for!(i = 0; i < CMPT_MF_HASH_TABLE_SIZE!(); i.suffix_plus_plus(); {
        let mut value: u32 = i.cast();
        c_for!(j = 0; j < 8; j.suffix_plus_plus(); {
            if value & 1 != 0 {
                value = (value >> 1) ^ poly32;
            } else {
                value >>= 1;
            }
        });
        hashRootTable[i] = value.cast();
    });
    return;
}
