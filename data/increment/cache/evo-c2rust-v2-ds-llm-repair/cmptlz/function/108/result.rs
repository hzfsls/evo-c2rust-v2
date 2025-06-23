pub fn CmptLzTryDecLitPacket(mut decCtx: Ptr<CmptLzDecCtx>, mut range: u32, mut rangeCode: u32, mut rangeBound: u32, mut bufTryDec: Ptr<u8>, mut pbufLimit: Ptr<Ptr<u8>>) -> i32 {
    let mut probBit: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut procPos: u32 = decCtx.processedPos;
    let mut litPosMask: u32 = ((0x100 as u32) << decCtx.prop.litPos) - ((0x100 as u32) >> decCtx.prop.litCtx);

    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut dicPos: usize = decCtx.dictPos;
    let mut dict: Ptr<u8> = decCtx.dict;
    let mut bufLimit: Ptr<u8> = *pbufLimit;

    if (decCtx.dictPos >= decCtx.dictBufSize) {
        return CMPT_ERROR_DATA!();
    }

    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if (procPos != 0) || (decCtx.checkDicSize != 0) {
        probSlot += (3 as u32) * ((((procPos << 8) + dict[(if dicPos == 0 { dictBufSize } else { dicPos }) - 1]) & litPosMask) << decCtx.prop.litCtx);
    }

    let mut decSym: u32 = 1;
    if (decCtx.state < CMPTLZ_LIT_STATES!()) {
        c_do!({
            probBit = (probSlot + decSym);
            CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        } while decSym < 0x100);
    } else {
        let mut bit: u32 = Default::default();
        let tmp0 = dicPos - decCtx.reps[0;
        let mut matchSym: u32 = dict[tmp0] + (if dicPos < decCtx.reps[0] { dictBufSize } else { 0 })].cast();
        let mut offset: u32 = 0x100;
        c_do!({
            matchSym <<= 1;
            bit = offset;
            offset &= matchSym;
            probBit = (probSlot + (offset + bit + decSym));
            CMPTLZ_MATCH_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        } while decSym < 0x100);
    }

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK!();
}
