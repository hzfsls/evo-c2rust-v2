pub fn CmptLzMatchDec(mut decCtx: Ptr<CmptLzDecCtx>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>, mut dicPosLimit: usize, mut posState: u32) -> u32 {
    let mut matchLen: u32 = Default::default();
    let mut matchDist: usize = Default::default();
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    probSlot = CmptLzGetMatchLenCoderProb(probsMatrix);
    matchLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);
    matchDist = CmptLzDistDec(decCtx, probsMatrix, pRange, pRangeCode, pRangeBound, matchLen);
    if (matchDist > decCtx.dictBufSize) {
        if (matchDist == 0xFFFFFFFF) {
            decCtx.remainLen = CMPTLZ_MATCH_MAX_LEN;
            decCtx.state -= CMPTLZ_MKSTATE_NUM;
            return CMPT_OK;
        } else {
            return CMPT_ERROR_DATA;
        }
    }
    return CmptLzDecByDistAndLen(decCtx, matchDist, (matchLen + 2), dicPosLimit);
}