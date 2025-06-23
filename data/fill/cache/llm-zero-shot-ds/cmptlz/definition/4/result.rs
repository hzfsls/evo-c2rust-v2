pub struct CmptLzMemHook {
    pub CmptLzAlloc: fn(enMemType: i32, size: usize) -> *mut std::ffi::c_void,
    pub CmptLzFree: fn(enMemType: i32, address: *mut std::ffi::c_void),
}
