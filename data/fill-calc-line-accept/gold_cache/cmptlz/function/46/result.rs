pub fn CmptLzRangeCodeInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut rangeCode: u32 = (decCtx.tempBuf[1] as u32) << 24;
    rangeCode |= (decCtx.tempBuf[2] as u32) << 16;
    rangeCode |= (decCtx.tempBuf[3] as u32) << 8;
    rangeCode |= decCtx.tempBuf[4] as u32;
    decCtx.code = rangeCode;
    decCtx.range = 0xFFFFFFFF;
}