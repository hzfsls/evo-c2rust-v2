pub fn BzpGetDictionaryList(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ninUse: i32 = 0;
    let mut use16: Array<bool, 16> = Default::default();
    let mut inUse: Array<bool, { BZP_ASCII_SIZE!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!(); i += 1; {
        use16[i as usize] = BzpReadBits(BZP_BIT!(), inData).cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!(); i += 1; {
        if use16[i as usize] {
            c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j += 1; {
                inUse[(i * BZP_GROUPS_ASCII!() + j) as usize] = BzpReadBits(BZP_BIT!(), inData).cast();
            });
        }
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!(); i += 1; {
        if inUse[i as usize] {
            inData.list[ninUse as usize] = i;
            ninUse += 1;
        }
    });
    return ninUse;
}