use std::ptr;

static RAPIDLZ_MAX_4BIT_VALUE: u32 = 0xF; // 15
static RAPIDLZ_MAX_BYTE_VALUE: u32 = 0xFF; // 255

fn rapidlz_store_off_match(dst: &mut [u8], token: &mut u8, match_length: u32, offset: u16) -> &mut [u8] {
    let mut dst_curr = dst;

    // Write offset in little-endian
    dst_curr[0] = offset as u8;
    dst_curr[1] = (offset >> 8) as u8;
    dst_curr = &mut dst_curr[2..];

    if match_length >= RAPIDLZ_MAX_4BIT_VALUE {
        let mut optional_len = match_length - RAPIDLZ_MAX_4BIT_VALUE;
        *token += RAPIDLZ_MAX_4BIT_VALUE as u8;
        
        while optional_len >= RAPIDLZ_MAX_BYTE_VALUE {
            dst_curr[0] = RAPIDLZ_MAX_BYTE_VALUE as u8;
            dst_curr = &mut dst_curr[1..];
            optional_len -= RAPIDLZ_MAX_BYTE_VALUE;
        }
        
        dst_curr[0] = optional_len as u8;
        dst_curr = &mut dst_curr[1..];
    } else {
        *token += match_length as u8;
    }

    dst_curr
}
