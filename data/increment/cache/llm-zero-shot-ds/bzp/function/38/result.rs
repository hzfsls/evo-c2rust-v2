pub unsafe extern "C" fn BzpBwtDecodeFinish(bwt: *mut BzpBwtDecodeInfo) {
    if !bwt.is_null() {
        let bwt = &mut *bwt;
        
        if !bwt.block.is_null() {
            libc::free(bwt.block as *mut libc::c_void);
            bwt.block = std::ptr::null_mut();
        }
        
        if !bwt.deCode.is_null() {
            libc::free(bwt.deCode as *mut libc::c_void);
            bwt.deCode = std::ptr::null_mut();
        }
        
        if !bwt.sorted.is_null() {
            libc::free(bwt.sorted as *mut libc::c_void);
            bwt.sorted = std::ptr::null_mut();
        }
        
        libc::free(bwt as *mut _ as *mut libc::c_void);
    }
}
