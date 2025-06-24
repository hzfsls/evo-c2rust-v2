pub fn VOS_Sha256Calc(
    mut pucInput: Ptr<u8>,
    mut uiInputLen: u32,
    mut pucOutput: Ptr<u8>,
    mut uiOutputLen: u32,
) {
    let mut stCtx: VOS_SHA256_CTX = Default::default();
    vosSha256Begin(c_ref!(stCtx));
    vosSha256Hash(pucInput, uiInputLen, c_ref!(stCtx));
    vosSha256End(pucOutput, uiOutputLen, c_ref!(stCtx));
}