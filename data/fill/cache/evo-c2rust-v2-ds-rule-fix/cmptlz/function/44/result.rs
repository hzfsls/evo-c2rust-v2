pub fn CmptLzDecRemWriteInDict(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize) {
    let mut dictPos: usize = decCtx.dictPos.cast();
    let mut remainDecLen: usize = decCtx.remainLen.cast();
    let mut dictBufSize: usize = decCtx.dictBufSize.cast();
    let mut remainDicLen: usize = dicPosLimit - dictPos;
    if (remainDicLen < remainDecLen).as_bool() {
        remainDecLen = remainDicLen.cast();
    }
    if (remainDecLen == 0).as_bool() {
        return;
    }
    decCtx.processedPos += remainDecLen.cast::<u32>();
    decCtx.remainLen -= remainDecLen.cast::<u32>();
    let mut dict: Ptr<u8> = decCtx.dict.cast();
    let mut rep0: usize = decCtx.reps[0].cast();
    while (remainDecLen != 0).as_bool() {
        remainDecLen -= 1;
        dict[dictPos] = dict[dictPos - rep0 + if dictPos < rep0 { dictBufSize } else { 0 }].cast();
        dictPos += 1;
    }
    decCtx.dictPos = dictPos.cast();
    CmptLzDecCheckDictSizeUpdate(decCtx.cast());
}
