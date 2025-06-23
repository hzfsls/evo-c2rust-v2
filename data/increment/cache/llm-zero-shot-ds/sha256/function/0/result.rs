use std::ptr;
use std::mem;

pub struct VosSha256Ctx {
    h: [u32; 8],
    outlen: usize,
}

pub const SHA256_DIGEST_SIZE: usize = 32;

pub fn vos_sha256_begin(pst_ctx: Option<&mut VosSha256Ctx>) {
    if pst_ctx.is_none() {
        return;
    }
    
    let ctx = pst_ctx.unwrap();
    
    // Zero out the entire struct
    unsafe {
        ptr::write_bytes(ctx, 0, 1);
    }
    
    // Initialize hash values
    ctx.h[0] = 0x6a09e667;
    ctx.h[1] = 0xbb67ae85;
    ctx.h[2] = 0x3c6ef372;
    ctx.h[3] = 0xa54ff53a;
    ctx.h[4] = 0x510e527f;
    ctx.h[5] = 0x9b05688c;
    ctx.h[6] = 0x1f83d9ab;
    ctx.h[7] = 0x5be0cd19;
    
    ctx.outlen = SHA256_DIGEST_SIZE;
}
