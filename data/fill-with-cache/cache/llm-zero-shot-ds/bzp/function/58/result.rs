use std::ptr;

#[repr(C)]
pub struct BzpBwtInfo {
    block: *mut u8,
    sortBlock: *mut u8,
    idx: *mut u32,
    isStartPos: *mut bool,
}

pub unsafe fn BzpBwtFinish(bwt: *mut BzpBwtInfo) {
    if !bwt.is_null() {
        let bwt_ref = &mut *bwt;
        
        if !bwt_ref.block.is_null() {
            libc::free(bwt_ref.block as *mut libc::c_void);
            bwt_ref.block = ptr::null_mut();
        }
        
        if !bwt_ref.sortBlock.is_null() {
            libc::free(bwt_ref.sortBlock as *mut libc::c_void);
            bwt_ref.sortBlock = ptr::null_mut();
        }
        
        if !bwt_ref.idx.is_null() {
            libc::free(bwt_ref.idx as *mut libc::c_void);
            bwt_ref.idx = ptr::null_mut();
        }
        
        if !bwt_ref.isStartPos.is_null() {
            libc::free(bwt_ref.isStartPos as *mut libc::c_void);
            bwt_ref.isStartPos = ptr::null_mut();
        }
        
        libc::free(bwt as *mut libc::c_void);
    }
}
