use std::ptr;

static mut CMPTLZ_REP3: usize = 3;
static mut CMPTLZ_REP2: usize = 2;
static mut CMPTLZ_LIT_STATES: u32 = 0;

struct CmptLzDecCtx {
    reps: [u32; 4],
    buf: *const u8,
    state: u32,
}

unsafe fn cmpt_lz_dist_dec_helper(
    dec_ctx: *mut CmptLzDecCtx,
    dist_dec: u32,
    buf_to_dec: *const u8,
    p_range: *mut u32,
    p_range_code: *mut u32,
    p_range_bound: *mut u32,
    range: u32,
    range_code: u32,
    range_bound: u32,
) {
    (*dec_ctx).reps[CMPTLZ_REP3] = (*dec_ctx).reps[CMPTLZ_REP2];
    (*dec_ctx).reps[CMPTLZ_REP2] = (*dec_ctx).reps[1];
    (*dec_ctx).reps[1] = (*dec_ctx).reps[0];
    (*dec_ctx).reps[0] = dist_dec + 1;
    (*dec_ctx).buf = buf_to_dec;
    (*dec_ctx).state = if (*dec_ctx).state < CMPTLZ_LIT_STATES {
        CMPTLZ_LIT_STATES
    } else {
        CMPTLZ_LIT_STATES + 3
    };
    ptr::write(p_range, range);
    ptr::write(p_range_code, range_code);
    ptr::write(p_range_bound, range_bound);
}
