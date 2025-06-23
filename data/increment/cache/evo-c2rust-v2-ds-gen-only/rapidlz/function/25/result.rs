pub fn RapidlzWriteLE16(mut addr: Ptr<Void>, mut val: u16) {
    if (RapidlzIsLE() != 0).as_bool() {
        *addr.cast::<Ptr<u16>>() = val.cast();
    } else {
        let mut tmp: Ptr<u8> = addr.cast::<Ptr<u8>>();
        tmp[0] = val.cast::<u8>();
        tmp[1] = (val >> 8).cast::<u8>();
    }
}
