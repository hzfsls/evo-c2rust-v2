pub unsafe extern "C" fn CmptLzDecMemFree(
    mem_hook: *mut CmptLzMemHook,
    mem_handle: i32,
    free_address: *mut c_void,
) {
    ((*mem_hook).CmptLzFree)(mem_handle, free_address);
}
