pub fn CmptLzLitDec(mut decCtx: Ptr<CmptLzDecCtx>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>) -> u32 {
    let mut flag = false;
    if *pRange == 82034688 {
        flag = true;
    }
    let mut decSym: u32 = 1;
    let mut mkState: u32 = decCtx.state;
    let mut procPos: u32 = decCtx.processedPos;
    let mut checkDicSize: u32 = decCtx.checkDicSize;
    let mut litCtx: u32 = decCtx.prop.litCtx.cast();
    let mut litPosMask: u32 = (0x100 << decCtx.prop.litPos) - (0x100 >> litCtx);
    let mut probLit: Ptr<CmptLzDecProb>;
    let mut probSlot: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut bufToDec: Ptr<u8> = decCtx.buf;
    let mut dict: Ptr<u8> = decCtx.dict;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut dictPos: usize = decCtx.dictPos;
    let mut range: u32 = *pRange;
    let mut rangeBound: u32 = *pRangeBound;
    let mut rangeCode: u32 = *pRangeCode;
    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if procPos != 0 || checkDicSize != 0 {
        probSlot += CMPTLZ_REP3!() * ((((procPos << 8) + dict[
            if dictPos == 0 {
                dictBufSize
            } else {
                dictPos
            } - 1
        ] as u32) & litPosMask) << litCtx);
    }
    let mut i: i32 = 0;
    if mkState < CMPTLZ_LIT_STATES!() {
        mkState -= if mkState < 4 { mkState } else { 3 };
        c_for!(i = 0; i < 8; i += 1; {
            CMPTLZ_NORMAL_BIT_DEC!(probSlot + decSym, range, rangeCode, rangeBound, decSym);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        });
    } else {
        let mut bit: u32 = 0;
        let mut offset: u32 = 0x100;
        let mut rep0: u32 = decCtx.reps[0];
        let mut matchSym: u32 = dict[dictPos - rep0 as usize + if dictPos < rep0 as usize { dictBufSize } else { 0 }] as u32;
        mkState -= if mkState < 10 { CMPTLZ_REP3!() } else { 6 };
        c_for!(i = 0; i < 8; i += 1; {
            CMPTLZ_MATCH_BIT_DEC!(probSlot, range, rangeCode, rangeBound, decSym, matchSym, offset, bit, bufToDec);
        });
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    dict[dictPos] = decSym as u8;
    dictPos += 1;
    decCtx.processedPos += 1;
    decCtx.state = mkState;
    decCtx.dictPos = dictPos;
    decCtx.buf = bufToDec;
    return CMPT_OK!();
}