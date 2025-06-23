pub fn CmptLzRangeCodeInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut rangeCode: u32 = (decCtx.tempBuf[1].cast::<u32>() << 24).cast();
    rangeCode |= (decCtx.tempBuf[2].cast::<u32>() << 16).cast();
    rangeCode |= (decCtx.tempBuf[3].cast::<u32>() << 8).cast();
    rangeCode |= decCtx.tempBuf[4].cast::<u32>();
    decCtx.code = rangeCode.cast();
    decCtx.range = 0xFFFFFFFF;
}
