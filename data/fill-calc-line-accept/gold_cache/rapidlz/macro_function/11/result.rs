macro_rules! RAPIDLZ_COMPRESSBOUND {
    ($size:expr) => {
        if ($size as u32) > RAPIDLZ_MAX_INPUT_SIZE!() {
            0
        } else {
            ($size as u32) + (($size as u32) / 255) + 16
        }
    };
}
pub(crate) use RAPIDLZ_COMPRESSBOUND;