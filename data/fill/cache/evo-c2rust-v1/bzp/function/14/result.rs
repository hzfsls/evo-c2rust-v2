pub fn BzpGetDictionaryList(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ninUse: i32 = 0;
    let mut use16: Array<bool, 16> = arr![false; 16];
    let mut inUse: Array<bool, { BZP_ASCII_SIZE!() }> = arr![false; BZP_ASCII_SIZE!()];
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!(); i.suffix_plus_plus(); {
        use16[i] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!(); i.suffix_plus_plus(); {
        if use16[i] {
            c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
                inUse[i * BZP_GROUPS_ASCII!() + j] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            });
        }
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        if inUse[i] {
            inData.list[ninUse.suffix_plus_plus()] = i.cast();
        }
    });
    return ninUse.cast();
}
