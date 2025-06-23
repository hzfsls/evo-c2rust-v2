pub fn CmptMemCmpByOneByte(mut buf1: Ptr<u8>, mut buf2: Ptr<u8>, mut len: u32, mut limit: u32) -> u32 {
    let mut lenIn: u32 = len;
    while (lenIn < limit).as_bool() && (buf1[lenIn] == buf2[lenIn]).as_bool() {
        lenIn += 1;
    }
    return lenIn.cast();
}
