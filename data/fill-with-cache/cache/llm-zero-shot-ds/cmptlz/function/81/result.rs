use std::ptr;

pub fn cmpt_mf_prepare(
    enc_ctx: &mut CmptLzEncCtx,
    src: *const u8,
    src_len: usize,
    alloc: &CmptLzMemHook,
) -> i32 {
    // Allocate memory for the match finder context
    let mf = unsafe { alloc.CmptLzAlloc(CMPTLZ_MF_CCTX_HANDLE, std::mem::size_of::<CmptMfCtx>()) };
    if mf.is_null() {
        return CMPT_ENC_MF_INIT_FAIL;
    }

    // Initialize the match finder context with zeros
    unsafe {
        ptr::write_bytes(mf, 0, 1);
    }

    let mf = unsafe { &mut *mf };
    enc_ctx.mf_ctx = mf;

    // Initialize match finder parameters
    mf.cycle_size = enc_ctx.dic_size + 1;
    let mut hash_mask = enc_ctx.dic_size - 1;
    cmpt_hash_mask_calc(&mut hash_mask);
    mf.hash_mask = hash_mask;

    let mut hash_mask = hash_mask + 1;
    hash_mask += CMPTLZ_HASH_2_SIZE;
    hash_mask += CMPTLZ_HASH_3_SIZE;
    mf.hash_count = hash_mask;
    mf.sons_count = mf.cycle_size * 2;

    // Allocate and initialize hash table
    mf.hash = unsafe { alloc.CmptLzAlloc(CMPTLZ_MF_HASH_HANDLE, mf.hash_count * std::mem::size_of::<u32>()) };
    if mf.hash.is_null() {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    unsafe {
        ptr::write_bytes(mf.hash, 0, mf.hash_count);
    }

    // Allocate and initialize son table
    mf.son = unsafe { alloc.CmptLzAlloc(CMPTLZ_MF_SON_HANDLE, mf.sons_count * std::mem::size_of::<u32>()) };
    if mf.son.is_null() {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    unsafe {
        ptr::write_bytes(mf.son, 0, mf.sons_count);
    }

    // Generate hash table and set remaining parameters
    cmptlz_mf_gen_hash_table(mf);
    mf.src_start = src;
    mf.src_len = src_len;
    mf.offset = mf.cycle_size;
    mf.nice_len = enc_ctx.num_fast_bytes;
    mf.depth = CMPT_MF_BASE_DEPTH + mf.nice_len / 2;

    0
}
