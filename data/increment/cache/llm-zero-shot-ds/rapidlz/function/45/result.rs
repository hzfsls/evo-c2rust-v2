use std::ptr;

const RAPIDLZ_MAX_4BIT_VALUE: u32 = 15;
const RAPIDLZ_MAX_BYTE_VALUE: u32 = 255;

// Assuming RapidlzCopy16Byte and RapidlzWildCopy16 are defined elsewhere
extern "C" {
    fn RapidlzCopy16Byte(dst: *mut u8, src: *const u8);
    fn RapidlzWildCopy16(src: *const u8, dst: *mut u8, end: *mut u8);
}

// Assuming RapidlzStoreOffMatch is defined elsewhere
extern "C" {
    fn RapidlzStoreOffMatch(dst: *mut u8, token: *mut u8, match_length: u32, offset: u16) -> *mut u8;
}

pub unsafe fn RapidlzStoreSequence(
    dst: *mut u8,
    src_anchor: *const u8,
    literal_length: u32,
    match_length: u32,
    offset: u16,
) -> *mut u8 {
    let mut dst_curr = dst;
    let token = dst_curr;
    dst_curr = dst_curr.offset(1);

    if literal_length >= RAPIDLZ_MAX_4BIT_VALUE {
        *token = (RAPIDLZ_MAX_4BIT_VALUE << 4) as u8;
        let mut optional_len = literal_length - RAPIDLZ_MAX_4BIT_VALUE;
        while optional_len >= RAPIDLZ_MAX_BYTE_VALUE {
            *dst_curr = RAPIDLZ_MAX_BYTE_VALUE as u8;
            dst_curr = dst_curr.offset(1);
            optional_len -= RAPIDLZ_MAX_BYTE_VALUE;
        }
        *dst_curr = optional_len as u8;
        dst_curr = dst_curr.offset(1);
        RapidlzCopy16Byte(dst_curr, src_anchor);
        if literal_length > 16 {
            RapidlzWildCopy16(
                src_anchor.add(16),
                dst_curr.add(16),
                dst_curr.add(literal_length as usize),
            );
        }
        dst_curr = dst_curr.add(literal_length as usize);
    } else if literal_length > 0 {
        *token = (literal_length << 4) as u8;
        RapidlzCopy16Byte(dst_curr, src_anchor);
        dst_curr = dst_curr.add(literal_length as usize);
    } else {
        *token = 0;
    }

    RapidlzStoreOffMatch(dst_curr, token, match_length, offset)
}
