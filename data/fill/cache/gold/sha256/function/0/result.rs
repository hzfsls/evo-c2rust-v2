pub fn vosSha256Begin(mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    if pstCtx == NULL!() {
        return;
    }
    c_memset_s!(
        pstCtx,
        c_sizeof!(VOS_SHA256_CTX),
        0,
        c_sizeof!(VOS_SHA256_CTX)
    );
    pstCtx.h[0] = 0x6a09e667;
    pstCtx.h[1] = 0xbb67ae85;
    pstCtx.h[2] = 0x3c6ef372;
    pstCtx.h[3] = 0xa54ff53a;
    pstCtx.h[4] = 0x510e527f;
    pstCtx.h[5] = 0x9b05688c;
    pstCtx.h[6] = 0x1f83d9ab;
    pstCtx.h[7] = 0x5be0cd19;
    pstCtx.outlen = SHA256_DIGEST_SIZE!();
}