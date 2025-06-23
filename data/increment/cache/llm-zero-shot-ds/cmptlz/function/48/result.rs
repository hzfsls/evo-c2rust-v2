use std::cmp::{min, max};

// Assuming the following constants are defined elsewhere in the Rust code
const CMPTLZ_MIN_DICTSIZE: u32 = /* value */;
const CMPTLZ_MAX_DICTSIZE: u32 = /* value */;
const CMPT_MF_LONGEST_MATCH: u32 = /* value */;
const CMPTLZ_LC_MAX: u32 = /* value */;
const CMPTLZ_LP_MAX: u32 = /* value */;
const CMPTLZ_PB_MAX: u32 = /* value */;

// Macro replacements as functions
fn cmptlz_set_dictsize_by_level(level: i32, dict_size: &mut u32) {
    *dict_size = if level <= 3 {
        1 << (level * 2 + 14)
    } else if level == 4 {
        1 << 21
    } else {
        1 << 22
    };
}

fn cmptlz_set_fb_by_level(level: i32, fast_bytes: &mut u32) {
    *fast_bytes = if level < 5 {
        32
    } else if level == 5 {
        40
    } else {
        64
    };
}

pub struct CmptlzEncParam {
    level: i32,
    dictSize: u32,
    fastBytes: u32,
    litCtx: i32,
    litPos: i32,
    posBits: i32,
    numThreads: u32,
}

pub fn cmptlz_param_normalize(props: &mut CmptlzEncParam) {
    // Normalize level
    props.level = props.level.clamp(0, 9);
    let level = props.level;

    // Normalize dictSize
    if props.dictSize < CMPTLZ_MIN_DICTSIZE || props.dictSize > CMPTLZ_MAX_DICTSIZE {
        cmptlz_set_dictsize_by_level(level, &mut props.dictSize);
    }

    // Normalize fastBytes
    if props.fastBytes < 5 || props.fastBytes > CMPT_MF_LONGEST_MATCH {
        cmptlz_set_fb_by_level(level, &mut props.fastBytes);
    }

    // Normalize other parameters
    props.litCtx = props.litCtx.clamp(0, CMPTLZ_LC_MAX as i32);
    props.litPos = props.litPos.clamp(0, CMPTLZ_LP_MAX as i32);
    props.posBits = props.posBits.clamp(0, CMPTLZ_PB_MAX as i32);

    // Set default number of threads
    props.numThreads = 1;
}
