pub fn CmptBtSkip(mut mf: Ptr<CmptMfCtx>, mut lenLimit: u32, mut pos: u32, mut cur: Ptr<u8>, mut curMatch: u32) {
    let mut depth: u32 = mf.depth.cast();
    let mut son: Ptr<u32> = mf.son.cast();
    let mut cyclePos: u32 = mf.cyclePos.cast();
    let mut cycleSize: u32 = mf.cycleSize.cast();
    let mut ptr0: Ptr<u32> = (son + (cyclePos << 1) + 1).cast();
    let mut ptr1: Ptr<u32> = (son + (cyclePos << 1)).cast();
    let mut len0: u32 = 0;
    let mut len1: u32 = 0;
    loop {
        let mut delta: u32 = (pos - curMatch).cast();
        if (depth.suffix_minus_minus() == 0 || delta >= cycleSize).as_bool() {
            *ptr0 = CMPT_EMPTY_HASH_VALUE!();
            *ptr1 = CMPT_EMPTY_HASH_VALUE!();
            return;
        }
        let mut pair: Ptr<u32> = (son + ((cyclePos - delta + if delta > cyclePos { cycleSize } else { 0 }) << 1)).cast();
        let mut pb: Ptr<u8> = (cur - delta).cast();
        let mut len: u32 = CMPTLZ_FIND_MIN!(len0, len1);
        if (pb[len] == cur[len]).as_bool() {
            len = CmptMemCmpLenSafe(pb.cast(), cur.cast(), (len + 1).cast(), lenLimit.cast()).cast();
            if (len == lenLimit).as_bool() {
                *ptr1 = pair[0].cast();
                *ptr0 = pair[1].cast();
                return;
            }
        }
        if (pb[len] < cur[len]).as_bool() {
            CMPT_MF_LEFT_SON_UPDATE!(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE!(ptr0, pair, curMatch, len0, len);
        }
    }
}
