use std::ptr;
use std::mem;

#[repr(C)]
pub struct CmptLzMemHook {
    CmptLzAlloc: extern "C" fn(u32, usize) -> *mut std::ffi::c_void,
}

#[repr(C)]
pub struct CmptLzEncCtx {
    endMarker: i32,
    rcCtx: *mut std::ffi::c_void,
    mfCtx: *mut std::ffi::c_void,
}

pub const CMPTLZ_ENC_CCTX_HANDLE: u32 = 0; // Assuming a value for the constant

pub unsafe extern "C" fn CmptInitCctx(alloc: *const CmptLzMemHook, writeEndMark: i32) -> *mut CmptLzEncCtx {
    if alloc.is_null() {
        return ptr::null_mut();
    }

    let alloc_fn = (*alloc).CmptLzAlloc;
    let handle = alloc_fn(CMPTLZ_ENC_CCTX_HANDLE, mem::size_of::<CmptLzEncCtx>());
    
    if handle.is_null() {
        return ptr::null_mut();
    }

    ptr::write_bytes(handle as *mut u8, 0, mem::size_of::<CmptLzEncCtx>());

    let enc_ctx = handle as *mut CmptLzEncCtx;
    (*enc_ctx).endMarker = writeEndMark;
    (*enc_ctx).rcCtx = ptr::null_mut();
    (*enc_ctx).mfCtx = ptr::null_mut();

    enc_ctx
}
