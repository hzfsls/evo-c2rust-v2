pub fn BzpMapInputChar(mut mtf: Ptr<BzpMtfInfo>, mut list: Ptr<u8>, mut lenList: i32) {
    if (BZP_ASCII_SIZE!() > lenList).as_bool() {
        return;
    }
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        if mtf.inUse[i].as_bool() {
            list[mtf.nUse.suffix_plus_plus()] = i.cast();
        }
    });
}