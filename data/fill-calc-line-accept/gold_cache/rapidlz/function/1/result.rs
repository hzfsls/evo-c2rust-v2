pub fn RapidlzReadLE16Bit(mut addr: VoidPtr) -> u16 {
    if RapidlzIsLE() != 0 {
        return *addr.cast::<Ptr<u16>>();
    }
    let mut tmp1: u8 = addr.cast::<Ptr<u8>>()[0];
    let mut tmp2: u8 = addr.cast::<Ptr<u8>>()[1];
    return tmp1 as u16 + ((tmp2 as u16) << 8);
}