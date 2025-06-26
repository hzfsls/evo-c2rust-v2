pub fn vosSha256CompressMul(
    mut pstCtx: Ptr<VOS_SHA256_CTX>,
    mut pucInput: Ptr<u8>,
    mut uiNum: u32,
) {
    let mut uiNumTmp: u32 = uiNum;
    let mut pucBlock: Ptr<u8> = pucInput;
    while uiNumTmp != 0 {
        vosSha256CompressBlock(pstCtx.h.cast(), pucBlock);
        pucBlock = pucBlock + SHA256_BLOCK_SIZE!();
        uiNumTmp -= 1;
    }
}