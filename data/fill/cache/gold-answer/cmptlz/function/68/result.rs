pub fn CmptHeadWrite(mut encCtx: Ptr<CmptLzEncCtx>, mut protData: Ptr<u8>, mut propsSize: Ptr<usize>) -> i32 {
    if protData == NULL!() {
        CMPTLZ_LOG!(CMPT_ERROR_DATA!(), cstr!("protData is NULL"));
        return CMPT_ENC_ERROR_HEAD!();
    }
    if *propsSize < CMPTLZ_PROPS_SIZE!() {
        CMPTLZ_LOG!(CMPT_ERROR_DATA!(), cstr!("propsSize need 5 bytes, get {}"), *propsSize);
        return CMPT_ENC_ERROR_HEAD!();
    }
    CmptlzWriteLE32Bit(protData + 1, encCtx.dicSize);
    protData[0] = ((encCtx.posBits * CMPTLZ_POS_STATE_MAX!() + encCtx.litPos) * CMPTLZ_LIT_CTX_MAX!() + encCtx.litCtx) as u8;
    *propsSize = CMPTLZ_PROPS_SIZE!();
    return 0;
}