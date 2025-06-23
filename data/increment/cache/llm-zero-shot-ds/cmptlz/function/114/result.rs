unsafe extern "C" fn CmptLzDecMemAlloc(memHook: *mut CmptLzMemHook, memHandle: i32, allocSize: usize) -> *mut c_void {
    if memHook.is_null() {
        return ptr::null_mut();
    }
    let hook = &*memHook;
    hook.CmptLzAlloc(memHandle, allocSize)
}
