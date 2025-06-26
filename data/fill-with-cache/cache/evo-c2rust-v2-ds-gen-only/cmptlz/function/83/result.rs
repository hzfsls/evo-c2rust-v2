pub fn CmptBtFind(mut mf: Ptr<CmptMfCtx>, mut curMatch: u32, mut matches: Ptr<CmptlzMatchPair>, mut longestLen: u32) -> Ptr<CmptlzMatchPair> {
    let mut depth: u32 = mf.depth;
    let mut son: Ptr<u32> = mf.son;
    let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast::<Ptr<u8>>();
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
        if (depth.suffix_minus_minus() == 0).as_bool() || (delta >= cycleSize).as_bool() {
            *ptr0 = CMPT_EMPTY_HASH_VALUE!();
            *ptr1 = CMPT_EMPTY_HASH_VALUE!();
            return matches;
        }
        let mut pair: Ptr<u32> = son + ((cyclePos - delta + if delta > cyclePos { cycleSize } else { 0 }) << 1;
        let mut pb: Ptr<u8> = (cur - delta).cast();
        let mut len: u32 = CMPTLZ_FIND_MIN!(len0, len1);
        if (pb[len] == cur[len]).as_bool() {
            len = CmptMemCmpLenSafe(pb.cast(), cur.cast(), len + 1, niceLen.cast()).cast();
            if (longestLen < len).as_bool() {
                longestLen = len;
                matches.len = len;
                matches.dist = delta - 1;
                matches = matches + 1;
                if (len == niceLen).as_bool() {
                    *ptr1 = pair[0];
                    *ptr0 = pair[1];
                    return matches;
                }
            }
        }
        if (pb[len] < cur[len]).as_bool() {
            CMPT_MF_LEFT_SON_UPDATE!(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE!(ptr0, pair, curMatch, len0, len);
        }
    }
}
