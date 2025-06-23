pub fn VOS_Sha256Calc(mut pucInput: Ptr<u8>, mut uiInputLen: u32, mut pucOutput: Ptr<u8>, mut uiOutputLen: u32) {
    let mut stCtx: VOS_SHA256_CTX = Default::default();

    vosSha256Begin(c_ref!(stCtx).cast());
    vosSha256Hash(pucInput.cast(), uiInputLen.cast(), c_ref!(stCtx).cast());
    vosSha256End(pucOutput.cast(), uiOutputLen.cast(), c_ref!(stCtx).cast());
}
