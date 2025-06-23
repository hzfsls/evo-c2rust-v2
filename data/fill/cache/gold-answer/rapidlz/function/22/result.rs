pub fn RapidlzStoreSequence(
    mut dst: Ptr<u8>,
    mut srcAnchor: Ptr<u8>,
    mut literalLength: u32,
    mut matchLength: u32,
    mut offset: u16,
) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst;
    let mut token: Ptr<u8> = dstCurr;
    dstCurr += 1;
    if literalLength >= RAPIDLZ_MAX_4BIT_VALUE!() {
        *token = (RAPIDLZ_MAX_4BIT_VALUE!() << 4).cast();
        let mut optionalLen: u32 = literalLength - RAPIDLZ_MAX_4BIT_VALUE!();
        c_for!(; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE!(); optionalLen -= RAPIDLZ_MAX_BYTE_VALUE!(); {
            *dstCurr = RAPIDLZ_MAX_BYTE_VALUE!().cast();
            dstCurr += 1;
        });
        *dstCurr = optionalLen.cast();
        dstCurr += 1;
        RapidlzCopy16Byte(dstCurr, srcAnchor);
        if literalLength > 16 {
            RapidlzWildCopy16(srcAnchor + 16, dstCurr + 16, dstCurr + literalLength);
        }
        dstCurr = dstCurr + literalLength;
    } else if literalLength > 0 {
        *token = (literalLength << 4).cast();
        RapidlzCopy16Byte(dstCurr, srcAnchor);
        dstCurr = dstCurr + literalLength;
    } else {
        *token = 0;
    }
    return RapidlzStoreOffMatch(dstCurr, token, matchLength, offset);
}