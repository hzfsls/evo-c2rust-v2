pub const RAPIDLZ_MAX_BYTE_VALUE: u32 = 255;

pub fn rapidlz_compress_store_optional_length(dst: &mut [u8], lit_length: u32) -> &mut [u8] {
    let mut dst_curr = dst;
    let mut length = lit_length;

    if length < RAPIDLZ_MAX_BYTE_VALUE {
        dst_curr[0] = length as u8;
        return &mut dst_curr[1..];
    }

    while length >= RAPIDLZ_MAX_BYTE_VALUE {
        dst_curr[0] = RAPIDLZ_MAX_BYTE_VALUE as u8;
        dst_curr = &mut dst_curr[1..];
        length -= RAPIDLZ_MAX_BYTE_VALUE;
    }

    dst_curr[0] = length as u8;
    &mut dst_curr[1..]
}
