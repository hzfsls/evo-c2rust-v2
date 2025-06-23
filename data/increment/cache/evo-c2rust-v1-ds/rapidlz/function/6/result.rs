pub fn RapidlzStoreMatchLen(mut matchLen: u32, mut token: Ptr<u8>, mut curDest: Ptr<u8>) -> usize {
    let mut curDestAnchor: Ptr<u8> = curDest.cast();
    if matchLen >= RAPIDLZ_MAX_4BIT_VALUE!() {
        *token += RAPIDLZ_MAX_4BIT_VALUE!();
        matchLen -= RAPIDLZ_MAX_4BIT_VALUE!();
        *curDest = RAPIDLZ_MAX_BYTE_VALUE!();
        while matchLen >= RAPIDLZ_MAX_BYTE_VALUE!() {
            *(curDest.prefix_plus_plus()) = RAPIDLZ_MAX_BYTE_VALUE!();
            matchLen -= RAPIDLZ_MAX_BYTE_VALUE!();
        }
        curDest += matchLen / RAPIDLZ_MAX_BYTE_VALUE!();
        *curDest = (matchLen % RAPIDLZ_MAX_BYTE_VALUE!()).cast::<u8>();
        curDest += 1;
    } else {
        *token += matchLen.cast::<u8>();
    }
    return (curDest - curDestAnchor).cast::<usize>();
}
