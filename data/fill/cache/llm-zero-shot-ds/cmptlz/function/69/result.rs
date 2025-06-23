use std::cmp::{min, max};

const CMPTLZ_MIN_DICTSIZE: u32 = 1 << 12;
const CMPTLZ_MAX_DICTSIZE: u32 = 1 << 27;
const CMPT_MF_LONGEST_MATCH: u32 = 273;
const CMPTLZ_LC_MAX: u32 = 8;
const CMPTLZ_LP_MAX: u32 = 4;
const CMPTLZ_PB_MAX: u32 = 4;

fn cmptlz_param_normalize(props: &mut CmptlzEncParam) {
    let level = props.level.clamp(0, 9);
    props.level = level;
    
    if props.dict_size < CMPTLZ_MIN_DICTSIZE || props.dict_size > CMPTLZ_MAX_DICTSIZE {
        props.dict_size = cmptlz_set_dictsize_by_level(level);
    }
    
    if props.fast_bytes < 5 || props.fast_bytes > CMPT_MF_LONGEST_MATCH {
        props.fast_bytes = cmptlz_set_fb_by_level(level);
    }
    
    props.lit_ctx = props.lit_ctx.clamp(0, CMPTLZ_LC_MAX as i32);
    props.lit_pos = props.lit_pos.clamp(0, CMPTLZ_LP_MAX as i32);
    props.pos_bits = props.pos_bits.clamp(0, CMPTLZ_PB_MAX as i32);
    
    props.num_threads = 1;
}

// Helper functions (assuming they exist or need to be implemented)
fn cmptlz_set_dictsize_by_level(level: i32) -> u32 {
    // Implementation based on the C macro CMPTLZ_SET_DICTSIZE_BY_LEVEL
    match level {
        0 | 1 => 1 << 16,
        2 | 3 => 1 << 20,
        4 | 5 => 1 << 21,
        6 => 1 << 22,
        7 => 1 << 23,
        8 | 9 => 1 << 24,
        _ => 1 << 21, // default case (shouldn't happen due to clamp)
    }
}

fn cmptlz_set_fb_by_level(level: i32) -> u32 {
    // Implementation based on the C macro CMPTLZ_SET_FB_BY_LEVEL
    match level {
        0 | 1 => 32,
        2 | 3 => 48,
        4 | 5 => 64,
        6 => 96,
        7 => 128,
        8 | 9 => 273,
        _ => 64, // default case (shouldn't happen due to clamp)
    }
}

// Assuming the CmptlzEncParam struct is defined as:
#[derive(Debug, Default)]
struct CmptlzEncParam {
    level: i32,
    dict_size: u32,
    fast_bytes: u32,
    lit_ctx: i32,
    lit_pos: i32,
    pos_bits: i32,
    num_threads: u32,
}
