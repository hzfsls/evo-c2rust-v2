pub fn RapidlzEncLastLiterals(mut curSrcAnchor: Ptr<u8>, mut srcEnd: Ptr<u8>, mut curDest: Ptr<u8>, mut destEnd: Ptr<u8>, mut destStart: Ptr<u8>) -> i32 {
    let mut lastLiteralsLen: usize = (srcEnd - curSrcAnchor);
    if (RAPIDLZ_LITERAL_LEN_COPY_END!(curDest, lastLiteralsLen) > destEnd) {
        return RAPIDLZ_ENC_NOT_OK!();
    }
    curDest += RapidlzStoreLiteralLen(lastLiteralsLen.cast(), curDest);
    if (RAPIDLZ_RETURN_IF_NOT_EOK!(c_memcpy_s!(curDest, (destEnd - curDest), curSrcAnchor, lastLiteralsLen))) {
        return RAPIDLZ_ENC_NOT_OK!();
    }
    curDest += lastLiteralsLen;
    return (curDest - destStart).cast();
}