use std::os::raw::c_int;

// Assuming these constants are defined somewhere
const CMPT_ERROR_UNSUPPORTED: c_int = -1;
const CMPT_OK: c_int = 0;

// Assuming these types are defined somewhere
#[repr(C)]
pub struct CmptLzDecCtx {
    // Context fields
}

#[repr(C)]
pub struct CmptLzMemHook {
    // Memory hook fields
}

// External functions that need to be linked
extern "C" {
    fn CmptLzDecFreeProbs(decCtx: *mut CmptLzDecCtx, memHook: *mut CmptLzMemHook);
    fn CmptLzFreeDict(decCtx: *mut CmptLzDecCtx, memHook: *mut CmptLzMemHook);
}

#[no_mangle]
pub extern "C" fn CmptLzDecFree(decCtx: *mut CmptLzDecCtx, memHook: *mut CmptLzMemHook) -> c_int {
    if decCtx.is_null() || memHook.is_null() {
        return CMPT_ERROR_UNSUPPORTED;
    }

    unsafe {
        CmptLzDecFreeProbs(decCtx, memHook);
        CmptLzFreeDict(decCtx, memHook);
    }

    CMPT_OK
}
