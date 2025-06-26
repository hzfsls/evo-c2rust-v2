use std::ptr;

pub struct BzpBwtDecodeInfo {
    block: *mut u8,
    deCode: *mut u8,
    sorted: *mut u8,
}

pub unsafe fn BzpBwtDecodeFinish(bwt: *mut BzpBwtDecodeInfo) {
    if !bwt.is_null() {
        let bwt_ref = &mut *bwt;
        
        if !bwt_ref.block.is_null() {
            libc::free(bwt_ref.block as *mut libc::c_void);
            bwt_ref.block = ptr::null_mut();
        }
        
        if !bwt_ref.deCode.is_null() {
            libc::free(bwt_ref.deCode as *mut libc::c_void);
            bwt_ref.deCode = ptr::null_mut();
        }
        
        if !bwt_ref.sorted.is_null() {
            libc::free(bwt_ref.sorted as *mut libc::c_void);
            bwt_ref.sorted = ptr::null_mut();
        }
        
        libc::free(bwt as *mut libc::c_void);
    }
}
