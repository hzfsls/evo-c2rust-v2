pub fn CmptlzMatchFinder(mut mf: Ptr<CmptMfCtx>, mut pCount: Ptr<u32>, mut matches: Ptr<CmptlzMatchPair>) -> u32 {
    if CMPTLZ_UNLIKELY!(mf.srcLen - mf.readPos < mf.niceLen).as_bool() {
        *pCount = 0;
        mf.readPos += 1;
        mf.readAhead += 1;
        return 0;
    }
    let mut count: u32 = CmptlzBt4Finder(mf.cast(), matches.cast()).cast();
    if (count == 0).as_bool() {
        *pCount = 0;
        mf.readAhead += 1;
        return 0;
    }
    let mut longestLen: u32 = matches[count - 1].len.cast();
    if (longestLen == mf.niceLen).as_bool() {
        let mut bytesAvail: u32 = CMPTLZ_FIND_MIN!(mf.srcLen - mf.readPos + 1, CMPT_MF_LONGEST_MATCH!()).cast();
        let mut p1: Ptr<u8> = (mf.srcStart + mf.readPos - 1).cast();
        let mut p2: Ptr<u8> = (p1 - matches[count - 1].dist - 1).cast();
        longestLen = CmptMemCmpLenSafe(p1.cast(), p2.cast(), longestLen.cast(), bytesAvail.cast()).cast();
    }
    *pCount = count.cast();
    mf.readAhead += 1;
    return longestLen.cast();
}
