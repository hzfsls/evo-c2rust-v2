use std::ptr;

pub struct BzpMtfInfo {
    mtfV: *mut u8, // Assuming mtfV is a pointer to some data; adjust the type as needed
}

pub unsafe fn BzpMtfFinish(mtf: *mut BzpMtfInfo) {
    if !mtf.is_null() {
        let mtf_ref = &mut *mtf;
        if !mtf_ref.mtfV.is_null() {
            // Free the memory pointed to by mtfV
            let _ = Box::from_raw(mtf_ref.mtfV);
            mtf_ref.mtfV = ptr::null_mut();
        }
        // Free the BzpMtfInfo structure itself
        let _ = Box::from_raw(mtf);
    }
}
