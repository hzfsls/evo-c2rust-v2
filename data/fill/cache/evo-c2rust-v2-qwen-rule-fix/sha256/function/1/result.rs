pub fn vosSha256CtxPrepare(mut pstCtx: Ptr<VOS_SHA256_CTX>, mut uiLen: u32) -> u32 {
    let mut uiCntFirst: u32;
    let mut uiCntSec: u32;
    uiCntFirst = (pstCtx.N[0] + (uiLen << SHIFTS_PER_BYTE!())).cast::<u32>() & 0xffffffff;
    if (uiCntFirst < pstCtx.N[0]).as_bool() {
        pstCtx.N[1] += 1;
        if (pstCtx.N[1] == 0).as_bool() {
            pstCtx.corrupted = 1;
            return SHA256_ERROR!();
        }
    }
    uiCntSec = pstCtx.N[1] + (uiLen >> (BITSIZE!(u32) - SHIFTS_PER_BYTE!()));
    if (uiCntSec < pstCtx.N[1]).as_bool() {
        pstCtx.corrupted = 1;
        return SHA256_ERROR!();
    }
    pstCtx.N[1] = uiCntSec;
    pstCtx.N[0] = uiCntFirst;
    return SHA256_OK!();
}