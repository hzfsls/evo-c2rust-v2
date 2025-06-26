pub unsafe fn BzpMtfFinish(mtf: *mut BzpMtfInfo) {
    if !mtf.is_null() {
        let mtf_ref = &mut *mtf;
        if !mtf_ref.mtfV.is_null() {
            libc::free(mtf_ref.mtfV as *mut libc::c_void);
            mtf_ref.mtfV = std::ptr::null_mut();
        }
        libc::free(mtf as *mut libc::c_void);
    }
}
