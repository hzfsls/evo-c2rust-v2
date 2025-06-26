pub fn CmptLzMatchDec(mut decCtx: Ptr<CmptLzDecCtx>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>, mut dicPosLimit: usize, mut posState: u32) -> u32 {
    let mut matchLen: u32 = Default::default();
    let mut matchDist: usize = Default::default();
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast()).cast();
    probSlot = CmptLzGetMatchLenCoderProb(probsMatrix.cast()).cast();
    matchLen = CmptLzLenDec(decCtx.cast(), probSlot.cast(), pRange.cast(), pRangeCode.cast(), pRangeBound.cast(), posState.cast()).cast();
    matchDist = CmptLzDistDec(decCtx.cast(), probsMatrix.cast(), pRange.cast(), pRangeCode.cast(), pRangeBound.cast(), matchLen.cast()).cast();
    if (matchDist > decCtx.dictBufSize.cast()).as_bool() {
        if (matchDist == 0xFFFFFFFF).as_bool() {
            decCtx.remainLen = CMPTLZ_MATCH_MAX_LEN.cast();
            decCtx.state -= CMPTLZ_MKSTATE_NUM.cast();
            return CMPT_OK.cast();
        } else {
            return CMPT_ERROR_DATA.cast();
        }
    }
    return CmptLzDecByDistAndLen(decCtx.cast(), matchDist.cast(), (matchLen + 2).cast(), dicPosLimit.cast()).cast();
}