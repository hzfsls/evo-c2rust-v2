pub fn CmptInitCctx(mut alloc: Ptr<CmptLzMemHook>, mut writeEndMark: i32) -> VoidPtr {
    let mut handle: VoidPtr = (alloc.CmptLzAlloc)(CMPTLZ_ENC_CCTX_HANDLE!(), c_sizeof!(CmptLzEncCtx).cast());
    if handle == NULL!() {
        return NULL!();
    }
    c_memset_s!(handle, c_sizeof!(CmptLzEncCtx), 0, c_sizeof!(CmptLzEncCtx));
    let mut encCtx: Ptr<CmptLzEncCtx> = handle.cast();
    encCtx.endMarker = writeEndMark;
    encCtx.rcCtx = NULL!();
    encCtx.mfCtx = NULL!();
    return encCtx.cast();
}