pub fn RapidlzStoreSequence(mut dst: Ptr<u8>, mut srcAnchor: Ptr<u8>, mut literalLength: u32, mut matchLength: u32, mut offset: u16) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut token: Ptr<u8> = dstCurr.cast();
    dstCurr += 1;
    if (literalLength >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        *token = (RAPIDLZ_MAX_4BIT_VALUE!() << 4).cast();
        let mut optionalLen: u32 = literalLength - RAPIDLZ_MAX_4BIT_VALUE!();
        while (optionalLen >= RAPIDLZ_MAX_BYTE_VALUE!()).as_bool() {
            *dstCurr = RAPIDLZ_MAX_BYTE_VALUE!().cast();
            dstCurr += 1;
            optionalLen -= RAPIDLZ_MAX_BYTE_VALUE!();
        }
        *dstCurr = optionalLen.cast();
        dstCurr += 1;
        RapidlzCopy16Byte(dstCurr.cast(), srcAnchor.cast());
        if (literalLength > 16).as_bool() {
            RapidlzWildCopy16(srcAnchor.cast::<Ptr<u8>>() + 16, dstCurr.cast::<Ptr<u8>>() + 16, dstCurr.cast::<Ptr<u8>>() + literalLength);
        }
        dstCurr += literalLength;
    } else if (literalLength > 0).as_bool() {
        *token = (literalLength << 4).cast();
        RapidlzCopy16Byte(dstCurr.cast(), srcAnchor.cast());
        dstCurr += literalLength;
    } else {
        *token = 0;
    }
    return RapidlzStoreOffMatch(dstCurr.cast(), token.cast(), matchLength.cast(), offset.cast()).cast();
}