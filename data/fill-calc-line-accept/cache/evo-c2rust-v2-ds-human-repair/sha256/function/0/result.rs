pub fn vosSha256Begin(mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    if (pstCtx == NULL!()) {
        return;
    }
    c_memset_s!(pstCtx, c_sizeof!(VOS_SHA256_CTX), 0, c_sizeof!(VOS_SHA256_CTX)).cast::<Void>();
    pstCtx.h[0] = 0x6a09e667u32;
    pstCtx.h[1] = 0xbb67ae85u32;
    pstCtx.h[2] = 0x3c6ef372u32;
    pstCtx.h[3] = 0xa54ff53au32;
    pstCtx.h[4] = 0x510e527fu32;
    pstCtx.h[5] = 0x9b05688cu32;
    pstCtx.h[6] = 0x1f83d9abu32;
    pstCtx.h[7] = 0x5be0cd19u32;
    pstCtx.outlen = SHA256_DIGEST_SIZE!();
}
