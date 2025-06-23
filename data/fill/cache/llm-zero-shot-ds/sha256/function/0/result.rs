use std::ptr;
use std::mem;

pub struct VosSha256Ctx {
    h: [u32; 8],
    outlen: usize,
}

pub const SHA256_DIGEST_SIZE: usize = 32;

pub fn vos_sha256_begin(pst_ctx: &mut VosSha256Ctx) {
    if pst_ctx.is_null() {
        return;
    }
    
    unsafe {
        ptr::write_bytes(pst_ctx, 0, 1);
    }
    
    pst_ctx.h[0] = 0x6a09e667;
    pst_ctx.h[1] = 0xbb67ae85;
    pst_ctx.h[2] = 0x3c6ef372;
    pst_ctx.h[3] = 0xa54ff53a;
    pst_ctx.h[4] = 0x510e527f;
    pst_ctx.h[5] = 0x9b05688c;
    pst_ctx.h[6] = 0x1f83d9ab;
    pst_ctx.h[7] = 0x5be0cd19;
    pst_ctx.outlen = SHA256_DIGEST_SIZE;
}
