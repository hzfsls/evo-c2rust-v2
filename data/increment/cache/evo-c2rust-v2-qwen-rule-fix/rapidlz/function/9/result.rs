pub fn RapidlzStreamEncLiterals(mut curSrc: Ptr<u8>, mut curSrcAnchor: Ptr<u8>, mut curDest: Ptr<Ptr<u8>>, mut destEnd: Ptr<u8>) -> bool {
    let mut litLen: u32 = (curSrc - curSrcAnchor).cast();
    if RAPIDLZ_UNLIKELY!(RAPIDLZ_LITERAL_LEN_COPY_END!(*curDest, litLen) > destEnd) {
        return false;
    }
    *curDest = (*curDest + RapidlzStoreLiteralLen(litLen, (*curDest)));
    RapidlzWildCopy8(curSrcAnchor, (*curDest), (*curDest + litLen));
    *curDest = (*curDest + litLen);
    return true;
}