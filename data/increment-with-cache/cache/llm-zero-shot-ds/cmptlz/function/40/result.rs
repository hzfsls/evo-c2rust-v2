use std::ptr;

#[repr(C)]
pub struct CmptlzCompParam {
    level: i32,
    dictSize: u32,
    litCtx: u32,
    litPos: u32,
    posBits: u32,
    fastBytes: u32,
    numThreads: u32,
    memHook: *mut CmptLzMemHook,
    protData: *mut u8,
    protSize: *mut usize,
}

#[repr(C)]
pub struct CmptlzEncParam {
    level: i32,
    dictSize: u32,
    litCtx: u32,
    litPos: u32,
    posBits: u32,
    fastBytes: u32,
    numThreads: u32,
}

pub const CMPT_ENC_ERROR_PARAM: i32 = -1;

pub unsafe extern "C" fn CmptlzCompress(
    src: *mut libc::c_void,
    srcSize: usize,
    dst: *mut libc::c_void,
    dstSize: *mut usize,
    param: *mut CmptlzCompParam,
) -> i32 {
    if src.is_null() && srcSize != 0 {
        return CMPT_ENC_ERROR_PARAM;
    }

    const END_MARKER: i32 = 0;

    let props = CmptlzEncParam {
        level: (*param).level,
        dictSize: (*param).dictSize,
        litCtx: (*param).litCtx,
        litPos: (*param).litPos,
        posBits: (*param).posBits,
        fastBytes: (*param).fastBytes,
        numThreads: (*param).numThreads,
    };

    CmptlzEncode(
        dst as *mut u8,
        dstSize,
        src as *const u8,
        srcSize,
        &props,
        (*param).protData,
        (*param).protSize,
        END_MARKER,
        (*param).memHook,
    )
}

// Assuming CmptlzEncode is defined elsewhere with the following signature:
extern "C" {
    fn CmptlzEncode(
        dst: *mut u8,
        dstSize: *mut usize,
        src: *const u8,
        srcSize: usize,
        props: *const CmptlzEncParam,
        protData: *mut u8,
        protSize: *mut usize,
        endMarker: i32,
        alloc: *mut CmptLzMemHook,
    ) -> i32;
}
