pub fn CmptBtSkip(mut mf: Ptr<CmptMfCtx>, mut lenLimit: u32, mut pos: u32, mut cur: Ptr<u8>, mut curMatch: u32) {
    let mut depth: u32 = mf.depth;
    let mut son: Ptr<u32> = mf.son;
    let mut cyclePos: u32 = mf.cyclePos;
    let mut cycleSize: u32 = mf.cycleSize;
    let mut ptr0: Ptr<u32> = son + (cyclePos << 1) + 1;
    let mut ptr1: Ptr<u32> = son + (cyclePos << 1);
    let mut len0: u32 = 0;
    let mut len1: u32 = 0;
    loop {
        let mut delta: u32 = pos - curMatch;
        if depth == 0 || delta >= cycleSize {
            *ptr0 = CMPT_EMPTY_HASH_VALUE!();
            *ptr1 = CMPT_EMPTY_HASH_VALUE!();
            return;
        }
        depth -= 1;
        let mut pair: Ptr<u32> = son + ((cyclePos - delta + ((delta > cyclePos) as u32 * cycleSize)) << 1);
        let mut pb: Ptr<u8> = cur - delta;
        let mut len: u32 = CMPTLZ_FIND_MIN!(len0, len1);
        if pb[len] == cur[len] {
            len = CmptMemCmpLenSafe(pb, cur, len + 1, lenLimit);
            if len == lenLimit {
                *ptr1 = pair[0];
                *ptr0 = pair[1];
                return;
            }
        }
        if pb[len] < cur[len] {
            CMPT_MF_LEFT_SON_UPDATE!(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE!(ptr0, pair, curMatch, len0, len);
        }
    }
}