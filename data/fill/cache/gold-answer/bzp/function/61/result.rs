pub fn BzpMapInputChar(mut mtf: Ptr<BzpMtfInfo>, mut list: Ptr<u8>, mut lenList: i32) {
    if BZP_ASCII_SIZE!() > lenList {
        return;
    }
    c_for!(let mut i = 0; i < BZP_ASCII_SIZE!(); i += 1; {
        if mtf.inUse[i] {
            list[mtf.nUse] = i.cast();
            mtf.nUse += 1;
        }
    });
}