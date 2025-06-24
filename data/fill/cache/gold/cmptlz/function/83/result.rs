pub fn CmptBtFind(mut mf: Ptr<CmptMfCtx>, mut curMatch: u32, mut matches: Ptr<CmptlzMatchPair>, mut longestLen: u32) -> Ptr<CmptlzMatchPair> {
    let mut depth: u32 = mf.depth;
    let mut son: Ptr<u32> = mf.son;
    let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast();
    let mut niceLen: u32 = mf.niceLen;
    let mut cyclePos: u32 = mf.cyclePos;
    let mut cycleSize: u32 = mf.cycleSize;
    let mut pos: u32 = mf.readPos + mf.offset;
    let mut ptr0: Ptr<u32> = son + (cyclePos << 1) + 1;
    let mut ptr1: Ptr<u32> = son + (cyclePos << 1);
    let mut len0: u32 = 0;
    let mut len1: u32 = 0;
    loop {
        let mut delta: u32 = pos - curMatch;
        if depth == 0 || delta >= cycleSize {
            *ptr0 = CMPT_EMPTY_HASH_VALUE!();
            *ptr1 = CMPT_EMPTY_HASH_VALUE!();
            return matches;
        }
        depth -= 1;
        let mut pair: Ptr<u32> = son + ((cyclePos - delta + ((delta > cyclePos) as u32 * cycleSize)) << 1);
        let mut pb: Ptr<u8> = cur - delta;
        let mut len: u32 = CMPTLZ_FIND_MIN!(len0, len1);
        if pb[len] == cur[len] {
            len = CmptMemCmpLenSafe(pb, cur, len + 1, niceLen);
            if longestLen < len {
                longestLen = len;
                matches.len = len;
                matches.dist = delta - 1;
                matches = matches + 1;
                if len == niceLen {
                    *ptr1 = pair[0];
                    *ptr0 = pair[1];
                    return matches;
                }
            }
        }
        if pb[len] < cur[len] {
            CMPT_MF_LEFT_SON_UPDATE!(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE!(ptr0, pair, curMatch, len0, len);
        }
    }
}