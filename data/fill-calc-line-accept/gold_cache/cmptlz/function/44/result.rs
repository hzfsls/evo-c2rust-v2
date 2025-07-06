pub fn CmptLzDecRemWriteInDict(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize) {
    let mut dictPos: usize = decCtx.dictPos;
    let mut remainDecLen: usize = decCtx.remainLen as usize;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut remainDicLen: usize = dicPosLimit - dictPos;
    if remainDicLen < remainDecLen {
        remainDecLen = remainDicLen;
    }
    if remainDecLen == 0 {
        return;
    }
    decCtx.processedPos += remainDecLen as u32;
    decCtx.remainLen -= remainDecLen as u32;
    let mut dict: Ptr<u8> = decCtx.dict.cast();
    let mut rep0: usize = decCtx.reps[0] as usize;
    while remainDecLen != 0 {
        remainDecLen -= 1;
        dict[dictPos] = dict[dictPos - rep0 + if dictPos < rep0 { dictBufSize } else { 0 }];
        dictPos += 1;
    }
    decCtx.dictPos = dictPos;
    CmptLzDecCheckDictSizeUpdate(decCtx);
}