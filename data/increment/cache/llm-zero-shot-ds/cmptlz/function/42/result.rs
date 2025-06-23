pub unsafe extern "C" fn CmptMemCmpLenSafe(buf1: *const u8, buf2: *const u8, len: u32, limit: u32) -> u32 {
    CmptMemCmpByOneByte(buf1, buf2, len, limit)
}
