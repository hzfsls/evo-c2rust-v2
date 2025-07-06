pub fn CmptlzMfGenHashTable(mut mf: Ptr<CmptMfCtx>) {
    let mut hashRootTable: Ptr<u32> = mf.hashRootTable.cast();
    let poly32: u32 = 0xEDB88320;
    let mut i: u32;
    let mut j: u32;
    c_for!(i = 0; i < CMPT_MF_HASH_TABLE_SIZE!(); i += 1; {
        let mut value: u32 = i;
        c_for!(j = 0; j < 8; j += 1; {
            if value & 1 != 0 {
                value = (value >> 1) ^ poly32;
            } else {
                value >>= 1;
            }
        });
        hashRootTable[i] = value;
    });
    return;
}