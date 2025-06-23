pub unsafe fn RapidlzCCtxFree(cCtx: *mut RapidlzCCtx) {
    if !cCtx.is_null() {
        let cCtx_ref = &mut *cCtx;
        if !cCtx_ref.hashTable.is_null() {
            libc::free(cCtx_ref.hashTable as *mut libc::c_void);
            cCtx_ref.hashTable = std::ptr::null_mut();
        }
        libc::free(cCtx as *mut libc::c_void);
    }
}
