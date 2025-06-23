use std::ptr;

#[repr(C)]
pub enum CmptLzRep {
    Rep0 = 0,
    Rep1 = 1,
    Rep2 = 2,
    Rep3 = 3,
}

pub const CMPTLZ_REP3: usize = CmptLzRep::Rep3 as usize;
pub const CMPTLZ_REP2: usize = CmptLzRep::Rep2 as usize;
pub const CMPTLZ_LIT_STATES: u32 = 7; // Assuming typical value, adjust if different

#[repr(C)]
pub struct CmptLzDecCtx {
    pub reps: [u32; 4],
    pub buf: *const u8,
    pub state: u32,
}

pub unsafe fn cmpt_lz_dist_dec_helper(
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
    // Update repetition distances
    (*dec_ctx).reps[CMPTLZ_REP3] = (*dec_ctx).reps[CMPTLZ_REP2];
    (*dec_ctx).reps[CMPTLZ_REP2] = (*dec_ctx).reps[1];
    (*dec_ctx).reps[1] = (*dec_ctx).reps[0];
    (*dec_ctx).reps[0] = dist_dec + 1;

    // Update buffer pointer and state
    (*dec_ctx).buf = buf_to_dec;
    (*dec_ctx).state = if (*dec_ctx).state < CMPTLZ_LIT_STATES {
        CMPTLZ_LIT_STATES
    } else {
        CMPTLZ_LIT_STATES + 3 // CMPTLZ_REP3 as u32
    };

    // Write output parameters
    ptr::write(p_range, range);
    ptr::write(p_range_code, range_code);
    ptr::write(p_range_bound, range_bound);
}
