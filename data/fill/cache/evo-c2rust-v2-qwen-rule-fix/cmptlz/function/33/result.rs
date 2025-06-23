pub fn CmptLzShortRepDec(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut rep0: u32 = decCtx.reps[0];
    let mut dict: Ptr<u8> = decCtx.dict;
    let mut dictPos: usize = decCtx.dictPos;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    dict[dictPos] = dict[dictPos - rep0 + if (dictPos < rep0) { dictBufSize } else { 0 }];
    decCtx.dictPos += 1;
    decCtx.processedPos += 1;
    if (decCtx.state < CMPTLZ_LIT_STATES!()) {
        decCtx.state = 9;
    } else {
        decCtx.state = 11;
    }
}