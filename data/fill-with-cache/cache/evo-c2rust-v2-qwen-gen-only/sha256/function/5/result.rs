pub fn vosSha256End(mut pucOut: Ptr<u8>, mut uiOutSize: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut uiIndex: u32 = Default::default();
    let mut pucBlock: Ptr<u8> = NULL!();
    let mut uiBlcLen: u32 = Default::default();
    if (pstCtx == NULL!()).as_bool() {
        return;
    }
    pucBlock = pstCtx.block.cast::<Ptr<u8>>();
    uiBlcLen = pstCtx.blocklen.cast();
    if (pstCtx.corrupted == 1).as_bool() || (uiOutSize < pstCtx.outlen).as_bool() {
        c_memset_s!(pstCtx.cast(), c_sizeof!(VOS_SHA256_CTX), 0, c_sizeof!(VOS_SHA256_CTX)).cast::<Void>();
        return;
    }
    if (pstCtx.computed == 0).as_bool() {
        pucBlock[uiBlcLen] = 0x80;
        uiBlcLen += 1;
        if (uiBlcLen > (SHA256_BLOCK_SIZE!() - 8).cast()).as_bool() {
            c_memset_s!(pucBlock + uiBlcLen, (SHA256_BLOCK_SIZE!() - uiBlcLen).cast(), 0, (SHA256_BLOCK_SIZE!() - uiBlcLen).cast()).cast::<Void>();
            uiBlcLen = 0;
            vosSha256CompressMul(pstCtx.cast(), pucBlock.cast(), 1);
        }
        c_memset_s!(pucBlock + uiBlcLen, (SHA256_BLOCK_SIZE!() - uiBlcLen).cast(), 0, (SHA256_BLOCK_SIZE!() - 8 - uiBlcLen).cast()).cast::<Void>();
        pucBlock = pucBlock + (SHA256_BLOCK_SIZE!() - 8);
        PUT_UINT32_BE!(pstCtx.N[1], pucBlock, 0);
        pucBlock = pucBlock + c_sizeof!(u32);
        PUT_UINT32_BE!(pstCtx.N[0], pucBlock, 0);
        pucBlock = pucBlock + c_sizeof!(u32);
        pucBlock = pucBlock - SHA256_BLOCK_SIZE!();
        vosSha256CompressMul(pstCtx.cast(), pucBlock.cast(), 1);
        pstCtx.blocklen = 0;
        c_memset_s!(pucBlock.cast(), SHA256_BLOCK_SIZE!(), 0, SHA256_BLOCK_SIZE!()).cast::<Void>();
        pstCtx.computed = 1;
    }
    uiBlcLen = ((pstCtx.outlen <= uiOutSize).as_bool().then(|| pstCtx.outlen).unwrap_or(uiOutSize) / c_sizeof!(u32)).cast();
    if (pucOut != NULL!()).as_bool() {
        c_for!(uiIndex = 0; uiIndex < uiBlcLen; uiIndex.suffix_plus_plus(); {
            PUT_UINT32_BE!(pstCtx.h[uiIndex], pucOut + (c_sizeof!(u32) * uiIndex), 0);
        });
    }
    return;
}