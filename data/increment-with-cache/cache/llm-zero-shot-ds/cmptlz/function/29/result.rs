use std::ptr::copy_nonoverlapping;

const CMPT_OK: i32 = 0;
const CMPT_ENC_ERROR_WRITE: i32 = -1; // Assuming this is the error code

struct CmptRcCtx {
    buf: *mut u8,
    bufBase: *mut u8,
    outBuf: *mut u8,
    outBufLeft: usize,
}

unsafe fn CmptRcFlush64Kb(rcCtx: *mut CmptRcCtx) -> i32 {
    let rcCtx = &mut *rcCtx;
    let flushOutLen = rcCtx.buf.offset_from(rcCtx.bufBase) as usize;
    
    if flushOutLen > rcCtx.outBufLeft {
        return CMPT_ENC_ERROR_WRITE;
    }
    
    copy_nonoverlapping(
        rcCtx.bufBase,
        rcCtx.outBuf,
        flushOutLen,
    );
    
    rcCtx.outBuf = rcCtx.outBuf.add(flushOutLen);
    rcCtx.outBufLeft -= flushOutLen;
    rcCtx.buf = rcCtx.bufBase;
    
    CMPT_OK
}
