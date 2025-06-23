pub unsafe fn BzpOutComDataFinish(data: *mut BzpOutComdata) {
    if !data.is_null() {
        let out = (*data).out;
        if !out.is_null() {
            libc::free(out as *mut libc::c_void);
            (*data).out = std::ptr::null_mut();
        }
        libc::free(data as *mut libc::c_void);
    }
}
