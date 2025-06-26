pub struct CmptLzDecCtx {
    dict: *mut u8,   // Assuming dict is a pointer to u8, adjust type if needed
    probs: *mut u8,  // Assuming probs is a pointer to u8, adjust type if needed
}

pub fn cmpt_lz_dec_construct(dec_ctx: &mut CmptLzDecCtx) {
    dec_ctx.dict = std::ptr::null_mut();
    dec_ctx.probs = std::ptr::null_mut();
}
