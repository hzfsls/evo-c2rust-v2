pub fn vosSha256Begin(mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    if (pstCtx == NULL!()).as_bool() {
        return;
    }
    c_memset_s!(pstCtx, c_sizeof!(VOS_SHA256_CTX), 0, c_sizeof!(VOS_SHA256_CTX)).cast::<Void>();
    pstCtx.h[0] = 0x6a09e667UL;
    pstCtx.h[1] = 0xbb67ae85UL;
    pstCtx.h[2] = 0x3c6ef372UL;
    pstCtx.h[3] = 0xa54ff53aUL;
    pstCtx.h[4] = 0x510e527fUL;
    pstCtx.h[5] = 0x9b05688cUL;
    pstCtx.h[6] = 0x1f83d9abUL;
    pstCtx.h[7] = 0x5be0cd19UL;
    pstCtx.outlen = SHA256_DIGEST_SIZE!();
}
