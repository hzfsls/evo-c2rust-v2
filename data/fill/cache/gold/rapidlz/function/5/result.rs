pub fn RapidlzWriteLE16(mut addr: VoidPtr, mut val: u16) {
    if RapidlzIsLE() != 0 {
        *addr.cast::<Ptr<u16>>() = val;
    } else {
        let mut tmp: Ptr<u8> = addr.cast();
        tmp[0] = val as u8;
        tmp[1] = (val >> 8) as u8;
    }
}