pub fn CmptlzMatchFinder(mut mf: Ptr<CmptMfCtx>, mut pCount: Ptr<u32>, mut matches: Ptr<CmptlzMatchPair>) -> u32 {
    if CMPTLZ_UNLIKELY!(mf.srcLen as u32 - mf.readPos < mf.niceLen) {
        *pCount = 0;
        mf.readPos += 1;
        mf.readAhead += 1;
        return 0;
    }
    let mut count: u32 = CmptlzBt4Finder(mf, matches);
    if count == 0 {
        *pCount = 0;
        mf.readAhead += 1;
        return 0;
    }
    let mut longestLen: u32 = matches[count - 1].len;
    if longestLen == mf.niceLen {
        let mut bytesAvail: u32 = CMPTLZ_FIND_MIN!(mf.srcLen as u32 - mf.readPos + 1 , CMPT_MF_LONGEST_MATCH!());
        let mut p1: Ptr<u8> = (mf.srcStart + mf.readPos - 1).cast();
        let mut p2: Ptr<u8> = (p1 - matches[count - 1].dist - 1).cast();
        longestLen = CmptMemCmpLenSafe(p1, p2, longestLen, bytesAvail);
    }
    *pCount = count;
    mf.readAhead += 1;
    return longestLen;
}