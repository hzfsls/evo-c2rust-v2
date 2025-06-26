pub fn RapidlzStoreMatchLen(mut matchLen: u32, mut token: Ptr<u8>, mut curDest: Ptr<u8>) -> usize {
    let mut curDestAnchor: Ptr<u8> = curDest.cast();
    if (matchLen >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        *token += RAPIDLZ_MAX_4BIT_VALUE!();
        matchLen -= RAPIDLZ_MAX_4BIT_VALUE!();
        *curDest = RAPIDLZ_MAX_BYTE_VALUE!();
        while (matchLen >= RAPIDLZ_MAX_BYTE_VALUE!()).as_bool() {
            *(curDest.suffix_plus_plus()) = RAPIDLZ_MAX_BYTE_VALUE!();
            matchLen -= RAPIDLZ_MAX_BYTE_VALUE!();
        }
        curDest += (matchLen / RAPIDLZ_MAX_BYTE_VALUE!()).cast();
        *curDest.suffix_plus_plus() = (matchLen % RAPIDLZ_MAX_BYTE_VALUE!()).cast::<u8>();
    } else {
        *token += matchLen.cast::<u8>();
    }
    return (curDest - curDestAnchor).cast();
}