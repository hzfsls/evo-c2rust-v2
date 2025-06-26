pub fn RapidlzReadLE16Bit(mut addr: Ptr<Void>) -> u16 {
    if (RapidlzIsLE() != 0).as_bool() {
        return (*addr.cast::<Ptr<u16>>()).cast();
    }
    let mut tmp1: u8 = ((addr.cast::<Ptr<u8>>())[0]).cast();
    let mut tmp2: u8 = ((addr.cast::<Ptr<u8>>())[1]).cast();
    return (tmp1 + (tmp2 << 8)).cast::<u16>();
}
