pub const RAPIDLZ_DEC_NOT_OK: i32 = /* value of RAPIDLZ_DEC_NOT_OK */;
pub const RAPIDLZ_ERROR_PARAM_UNSUPPORTED: i32 = /* value of RAPIDLZ_ERROR_PARAM_UNSUPPORTED */;

pub fn rapidlz_zero_bytes_decode(src: &[u8], src_size: usize) -> i32 {
    if !src.is_empty() && src[0] == 0 && src_size == 1 {
        return RAPIDLZ_DEC_NOT_OK;
    }
    RAPIDLZ_ERROR_PARAM_UNSUPPORTED
}
