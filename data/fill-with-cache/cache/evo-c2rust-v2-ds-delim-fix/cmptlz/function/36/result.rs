pub fn CmptLzLitDec(mut decCtx: Ptr<CmptLzDecCtx>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>) -> u32 {
    let mut decSym: u32 = 1;
    let mut mkState: u32 = decCtx.state.cast();
    let mut procPos: u32 = decCtx.processedPos.cast();
    let mut checkDicSize: u32 = decCtx.checkDicSize.cast();
    let mut litCtx: u32 = decCtx.prop.litCtx.cast();
    let mut litPosMask: u32 = ((0x100 as u32) << decCtx.prop.litPos) - ((0x100 as u32) >> litCtx);
    let mut probLit: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast());
    let mut bufToDec: Ptr<u8> = decCtx.buf.cast();
    let mut dict: Ptr<u8> = decCtx.dict.cast();
    let mut dictBufSize: usize = decCtx.dictBufSize.cast();
    let mut dictPos: usize = decCtx.dictPos.cast();
    let mut range: u32 = *pRange;
    let mut rangeBound: u32 = *pRangeBound;
    let mut rangeCode: u32 = *pRangeCode;
    probSlot = CmptLzGetLiteralProb(probsMatrix.cast());
    if (procPos != 0).as_bool() || (checkDicSize != 0).as_bool() {
        probSlot += (CMPTLZ_REP3!() as u32) *
                    ((((procPos << 8) + dict[(if dictPos == 0 { dictBufSize } else { dictPos }) - 1]) & litPosMask) << litCtx);
    }
    let mut i: i32 = 0;
    if (mkState < CMPTLZ_LIT_STATES!()).as_bool() {
        mkState -= if (mkState < 4).as_bool() { mkState } else { 3 };
        c_for!(; i < 8; i.suffix_plus_plus(); {
            CMPTLZ_NORMAL_BIT_DEC!((probSlot + decSym), range, rangeCode, rangeBound, decSym);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        });
    } else {
        let mut bit: u32 = Default::default();
        let mut offset: u32 = 0x100;
        let mut rep0: u32 = decCtx.reps[0].cast();
        let mut matchSym: u32 = dict[dictPos - rep0 + (if (dictPos < rep0).as_bool() { dictBufSize } else { 0 })].cast();
        mkState -= if (mkState < 10).as_bool() { CMPTLZ_REP3!() } else { 6 };
        c_for!(; i < 8; i.suffix_plus_plus(); {
            CMPTLZ_MATCH_BIT_DEC!(probSlot, range, rangeCode, rangeBound, decSym, matchSym, offset, bit, bufToDec);
        });
    }
    *pRange = range.cast();
    *pRangeCode = rangeCode.cast();
    *pRangeBound = rangeBound.cast();
    dict[dictPos] = decSym.cast::<u8>();
    dictPos += 1;
    decCtx.processedPos += 1;
    decCtx.state = mkState.cast();
    decCtx.dictPos = dictPos.cast();
    decCtx.buf = bufToDec.cast();
    return CMPT_OK!();
}
